<script setup lang="ts">
import ExecutableSelector from '@/components/ZipPreview/ExecutableSelector.vue'
import { goTo } from '@/router'
import { exec } from '@/exec'
import Button from 'primevue/button'
import Dialog from 'primevue/dialog'
import Drawer from 'primevue/drawer'
import InputText from 'primevue/inputtext'
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { generalStore, installConfig, libraryStore } from '@/main'
import { FileTreeNode } from '#/FileTreeNode'

const { t } = useI18n()
const showPasswordDialog = ref(false)
const archivePassword = ref('')
const passwordError = ref(false)
const detailsLoading = ref(false)
const subscribeSuccess = ref(false)
const isLoading = ref(false)

// Check if we're in temp mode (temporary installation from external)
const isTemporaryMode = computed(() => {
    return installConfig.temp.zip_path !== ''
})

// Computed properties
const drawerVisible = computed({
    get: () => generalStore.drawer.preview,
    set: (value: boolean) => {
        generalStore.drawer.preview = value
        // Clear data when drawer is closed without confirmation
        if (!value) {
            if (isTemporaryMode.value) {
                installConfig.clearTempData()
            } else {
                // For normal mode, clear file tree to allow fresh loading next time
                installConfig.file_tree = []
            }
        }
    }
})

// Handle button actions
function handleExecutableSelected() {
    if (isTemporaryMode.value) {
        // Confirm temp data when user selects executable
        installConfig.confirmTempData()
    }
    generalStore.drawer.preview = false
    goTo('/Install/App/Config')
}

function handleNoExecutable() {
    if (isTemporaryMode.value) {
        // Confirm temp data when user selects no executable
        installConfig.confirmTempData()
    }
    generalStore.drawer.preview = false
    goTo('/Install/Tool/Config')
}

async function handleSubscribe() {
    const url = isTemporaryMode.value ? installConfig.temp.url : installConfig.url
    if (!url) return

    subscribeSuccess.value = false

    // Add URL to subscribed URLs
    const timestamp = await exec<bigint>('GetTimestamp')

    libraryStore.urls.push({ url, timestamp })
    await libraryStore.saveLibrary()

    subscribeSuccess.value = true
}

function handleDetailsLoading(loading: boolean) {
    detailsLoading.value = loading
}

// Watch for drawer visibility changes to load content
watch(drawerVisible, async (visible) => {
    if (visible) {
        // Check if URL is already subscribed when drawer opens
        const url = isTemporaryMode.value ? installConfig.temp.url : installConfig.url
        if (url) {
            subscribeSuccess.value = libraryStore.urls.some(urlObj => urlObj.url === url)
        }

        const zipPath = isTemporaryMode.value ? installConfig.temp.zip_path : installConfig.zip_path

        // Always load archive content when drawer opens if we have a zip path
        // This ensures fresh content is loaded even if file_tree exists from previous operations
        if (zipPath) {
            try {
                isLoading.value = true
                await GetArchiveContent('')
            } catch (error) {
                if (error === 'Wrong password') {
                    showPasswordDialog.value = true
                } else {
                    globalThis.$errorHandler.showError(error)
                    generalStore.drawer.preview = false
                }
            } finally {
                isLoading.value = false
            }
        }
    }
})

// Watch for password changes to reset error
watch(archivePassword, () => {
    passwordError.value = false
})

async function handlePasswordSubmit() {
    if (!archivePassword.value) {
        passwordError.value = true
        return
    }

    try {
        await GetArchiveContent(archivePassword.value)
        if (isTemporaryMode.value) {
            installConfig.setTempData({ archive_password: archivePassword.value })
        } else {
            installConfig.archive_password = archivePassword.value
        }
        showPasswordDialog.value = false
        archivePassword.value = ''
    } catch (error) {
        if (error === 'Wrong password') {
            passwordError.value = true
        } else {
            globalThis.$errorHandler.showError(error)
            showPasswordDialog.value = false
            generalStore.drawer.preview = false
        }
    }
}

async function GetArchiveContent(password: string) {
    const zipPath = isTemporaryMode.value ? installConfig.temp.zip_path : installConfig.zip_path
    const treeData = await exec<FileTreeNode[]>('GetArchiveTree', {
        path: zipPath,
        password: password,
    })

    if (isTemporaryMode.value) {
        installConfig.setTempData({ file_tree: treeData })
    } else {
        installConfig.file_tree = treeData
    }
}
</script>

<template>
    <Drawer v-model:visible="drawerVisible" position="bottom" :style="{ height: '95vh' }" class="rounded-lg"
        :header="t('ui.preview.header')">
        <!-- Password Dialog -->
        <Dialog v-model:visible="showPasswordDialog" :modal="true" :closable="false"
            :header="t('ui.archive.password_required')" class="w-[30rem]">
            <div class="flex flex-col gap-3">
                <p class="text-sm">{{ t('ui.archive.enter_password') }}</p>
                <div class="flex flex-col gap-1">
                    <InputText v-model="archivePassword" type="password" :class="{ 'border-red-500': passwordError }"
                        @keydown.enter="handlePasswordSubmit" class="w-full" :invalid="passwordError" />
                    <small v-if="passwordError" class="text-red-500">{{ t('ui.archive.password_required') }}</small>
                </div>
            </div>
            <template #footer>
                <div class="flex justify-end gap-2">
                    <Button @click="generalStore.drawer.preview = false" :label="t('g.cancel')" severity="secondary"
                        outlined icon="mir-close" />
                    <Button @click="handlePasswordSubmit" :label="t('g.submit')" icon="mir-check" />
                </div>
            </template>
        </Dialog>

        <!-- Main content area -->
        <div class="flex h-full flex-col gap-4">
            <!-- File info header -->
            <div v-if="(isTemporaryMode ? installConfig.temp.zip_path : installConfig.zip_path)"
                class="flex items-center justify-between">
                <div class="flex flex-col gap-1">
                    <h3 class="text-lg font-semibold">
                        {{ (isTemporaryMode ? installConfig.temp.zip_path : installConfig.zip_path).split(/[/\\]/).pop()
                        }}
                    </h3>
                    <p class="text-sm">{{ (isTemporaryMode ? installConfig.temp.url :
                        installConfig.url) ||
                        (isTemporaryMode ? installConfig.temp.zip_path : installConfig.zip_path)
                        }}
                    </p>
                </div>

                <!-- Subscribe button -->
                <Button v-if="(isTemporaryMode ? installConfig.temp.url : installConfig.url)" @click="handleSubscribe"
                    :label="subscribeSuccess ? t('ui.preview.subscribed') : t('ui.preview.subscribe')"
                    :icon="subscribeSuccess ? 'mir-check' : 'mir-star'"
                    :severity="subscribeSuccess ? 'success' : 'secondary'" outlined class="shrink-0" />
            </div>

            <!-- ExecutableSelector embedded directly -->
            <div class="relative min-h-0 flex-1">
                <!-- Loading Overlay -->
                <div v-if="isLoading" class="absolute inset-0 z-50 flex items-center justify-center">
                    <div class="flex flex-col items-center gap-3">
                        <div class="flex items-center gap-2">
                            <div class="size-5 animate-spin rounded-full border-2 border-primary border-t-transparent">
                            </div>
                            <span class="text-sm font-medium">{{ t('g.loading')
                            }}</span>
                        </div>
                    </div>
                </div>

                <ExecutableSelector v-if="(isTemporaryMode ? installConfig.temp.zip_path : installConfig.zip_path)
                    && (isTemporaryMode ? installConfig.temp.file_tree : installConfig.file_tree).length > 0"
                    :zip-path="(isTemporaryMode ? installConfig.temp.zip_path : installConfig.zip_path)"
                    :password="(isTemporaryMode ? installConfig.temp.archive_password : installConfig.archive_password)"
                    :file-tree="(isTemporaryMode ? installConfig.temp.file_tree : installConfig.file_tree)"
                    :details-loading="detailsLoading" @loading="handleDetailsLoading"
                    @executable-selected="handleExecutableSelected" @no-executable="handleNoExecutable"
                    @update-file-tree="isTemporaryMode ? installConfig.setTempData({ file_tree: $event }) : (installConfig.file_tree = $event)" />
            </div>
        </div>
    </Drawer>
</template>
