<script setup lang="ts">
import { exec } from '@/exec'
import { generalStore, libraryStore } from '@/main'
import { goTo } from '@/router'
import Button from 'primevue/button'
import Checkbox from 'primevue/checkbox'
import InputText from 'primevue/inputtext'
import Panel from 'primevue/panel'
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'


const { t } = useI18n();
const id_modifying = ref(generalStore.drawer.tool_modify[1]);
const name = ref('');
const add_to_path = ref([false, '']);
const parent_install_path = ref('');
const pathError = ref(false);
const nameError = ref(false);
watch(() => generalStore.drawer.tool_modify[0], async (visible) => {
    if (visible) {
        id_modifying.value = generalStore.drawer.tool_modify[1];
        if (id_modifying.value) {
            const tool = await libraryStore.getTool(id_modifying.value);
            if (tool) {
                name.value = tool.details.name;
                add_to_path.value[0] = tool.details.add_to_path[0];
                add_to_path.value[1] = tool.details.add_to_path[1];
                parent_install_path.value = tool.details.install_path.replace(/\\[^\\]+$/, '');
            }
        }
    }
});
async function handleModifyClick() {
    pathError.value = false;
    nameError.value = false;
    nameError.value = !name.value;
    pathError.value = !parent_install_path.value;
    let install_path = '';
    try {
        const validatedPath = (await exec<string>('ValidatePath', {
            path: parent_install_path.value,
        }));
        parent_install_path.value = validatedPath;
        install_path = `${parent_install_path.value}\\${name.value}`;
    } catch (error) {
        globalThis.$errorHandler.showError(error);
        pathError.value = true;
    }
    if (nameError.value || pathError.value) {
        return;
    }
    try {
        const newTool = {
            id: id_modifying.value,
            timestamp_add: '',
            timestamp_update: '',
            installed: true,
            url: '',
            archive_password: '',
            details: {
                name: name.value,
                add_to_path: [add_to_path.value[0], add_to_path.value[1]],
                install_path: install_path,
            },
            validation_status: {
                file_exists: false,
                path_exists: false,
            },
        };
        await exec('ModifyTool', { new_tool: newTool, id: id_modifying.value });
        generalStore.drawer.tool_modify[1] = '';
        goTo('/Home');
    } catch (error) {
        globalThis.$errorHandler.showError(error);
    }
}
</script>

<template>
    <Drawer v-model:visible="generalStore.drawer.tool_modify[0]" position="bottom" :style="{ height: '60vh' }"
        class="rounded-lg">
        <template #header>
            <div class="flex items-center gap-2">
                <span class="mir-folder_copy text-xl"></span>
                <div>
                    <h3 class="text-lg font-medium">{{ t('ui.install.tool_details') }}</h3>
                </div>
            </div>
        </template>
        <div class="flex flex-wrap gap-4 px-1 md:flex-nowrap">
            <div class="min-w-72 flex-1 space-y-2">
                <Panel class="shadow-sm">
                    <template #header>
                        <div class="flex flex-col">
                            <div class="flex items-center gap-1.5">
                                <span class="mir-folder_copy text-lg" />
                                <h2 class="text-base font-medium">
                                    {{ t('ui.install.tool_details') }}
                                </h2>
                            </div>
                        </div>
                    </template>
                    <div class="space-y-4 p-1">
                        <div class="flex items-center gap-2">
                            <label class="w-24 text-sm font-medium">
                                {{ t('cls.app.name') }}
                            </label>
                            <div class="w-full">
                                <InputText v-model="name" :placeholder="t('cls.app.name')" class="h-8 w-full text-sm"
                                    :invalid="nameError" />
                            </div>
                        </div>
                        <div class="flex items-center gap-2">
                            <label class="w-24 text-sm font-medium">{{ t('cls.install.config.install_path') }}</label>
                            <div class="w-full">
                                <InputText v-model="parent_install_path" :placeholder="t('g.browse')"
                                    class="h-8 w-full text-sm" :invalid="pathError" />
                            </div>
                        </div>
                        <div class="mt-2 flex items-start gap-2">
                            <label class="mt-1 w-24 text-sm font-medium">{{ t('ui.install.environment') }}</label>
                            <div class="w-full">
                                <div class="flex-1 space-y-1 rounded-lg p-1.5">
                                    <div class="flex flex-col gap-1">
                                        <div class="flex items-center gap-2">
                                            <Checkbox v-model="add_to_path[0]" :binary="true" inputId="add_to_path" />
                                            <label for="add_to_path" class="text-sm">{{
                                                t('cls.install.shortcuts.add_to_path')
                                                }}</label>
                                        </div>
                                        <div v-if="add_to_path[0]" class="ml-6 mt-1">
                                            <InputText :value="typeof add_to_path[1] === 'string' ? add_to_path[1] : ''"
                                                @input="add_to_path[1] = ($event.target as HTMLInputElement).value"
                                                :placeholder="t('ui.select_placeholder.path_directory')"
                                                class="h-8 w-full text-sm" />
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
        <div class="flex items-center justify-end px-4 py-3">
            <Button severity="primary" class="h-8 w-28 text-sm transition-all duration-200" @click="handleModifyClick"
                icon="mir-install_desktop" :label="t('g.modify')" />
        </div>
    </Drawer>
</template>
