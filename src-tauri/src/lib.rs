use std::{error::Error, process::Command};

pub mod command;
pub mod installation;
pub mod settings;

pub fn elevate(revert: bool) -> Result<(), Box<dyn Error>> {
    let operation: &str;
    if !revert {
        operation = "Set-ItemProperty -Path $regPath -Name $programPath -Value $adminFlag";
    } else {
        operation =
            "Remove-ItemProperty -Path $regPath -Name $programPath -ErrorAction SilentlyContinue";
    }
    let ps_command = format!(
        r#"
        $currentUserSid = (Get-WmiObject Win32_UserAccount | Where-Object {{$_.Name -eq $env:USERNAME}}).SID

        $programPath = "{}"
        $regPath = "Registry::HKEY_USERS\$currentUserSid\Software\Microsoft\Windows NT\CurrentVersion\AppCompatFlags\Layers"
            
        if (-not (Test-Path $regPath)) {{
            New-Item -Path $regPath -Force | Out-Null
        }}
            
        $adminFlag = "~ RUNASADMIN"
        {}
        "#,
        std::env::current_exe()?.to_string_lossy(),
        operation
    );
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &ps_command,
        ])
        .output()?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }
    println!("{:#?}", std::env::current_exe()?.to_string_lossy());
    Ok(())
}
