use serde_json::{json, Value};
use std::error::Error;
use std::fs::File;
use std::process::Command;
use tempfile::Builder;
use zip::ZipArchive;

pub mod process;
pub mod setup;

pub fn get_details(arg: Vec<String>) -> Result<String, Box<dyn Error>> {
    let zip_path = &arg[0];
    let exe_path_in_zip = &arg[1];

    // Create temp directory
    let temp_dir = Builder::new().prefix("appporter").tempdir()?;

    // Extract the whole zip to temp directory using direct ZipArchive extract
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    archive.extract(temp_dir.path())?;

    // Get the path of extracted exe
    let temp_exe_path = temp_dir.path().join(exe_path_in_zip);

    // Prepare PowerShell command with error handling
    let ps_command = format!(
        "$ErrorActionPreference = 'Stop';
        try {{
            $file_path = '{}';
            $version_info = (Get-Item $file_path).VersionInfo;
            $product_name = if ($version_info.ProductName -and $version_info.ProductName.Trim()) {{ 
                $version_info.ProductName.Trim() 
            }} elseif ($version_info.FileDescription -and $version_info.FileDescription.Trim()) {{ 
                $version_info.FileDescription.Trim() 
            }} else {{ 
                [System.IO.Path]::GetFileNameWithoutExtension($file_path) 
            }};
            $product_version = if ($version_info.ProductVersion -and $version_info.ProductVersion.Trim()) {{ 
                $version_info.ProductVersion.Trim() 
            }} elseif ($version_info.FileVersion -and $version_info.FileVersion.Trim()) {{ 
                $version_info.FileVersion.Trim() 
            }} else {{ 
                'Unknown Version' 
            }};
            $copyright = if ($version_info.LegalCopyright -and $version_info.LegalCopyright.Trim()) {{ 
                $version_info.LegalCopyright.Trim() 
            }} else {{ 
                'No Copyright Information' 
            }};
            
            ConvertTo-Json @{{
               product_name=$product_name;
               product_version=$product_version;
               copyright=$copyright;
            }}
        }} catch {{
            ConvertTo-Json @{{
               error=$_.Exception.Message;
               product_name=[System.IO.Path]::GetFileNameWithoutExtension($file_path);
               product_version='Unknown Version';
               copyright='No Copyright Information';
            }}
        }}",
        temp_exe_path.to_string_lossy().replace("'", "''")
    );

    // Execute PowerShell command
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &ps_command])
        .output()?;

    // Parse the raw JSON response
    let details: Value = serde_json::from_slice(&output.stdout)?;

    // Convert to simple array format
    let simplified = json!([
        details["product_name"].as_str().unwrap_or("").trim(),
        details["product_version"].as_str().unwrap_or("").trim(),
        details["copyright"]
            .as_str()
            .unwrap_or("")
            .trim_start_matches("(C)")
            .trim_start_matches("©")
            .trim()
    ]);

    Ok(simplified.to_string())
}
