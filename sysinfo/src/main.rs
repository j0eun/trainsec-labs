use std::{ffi::OsString, os::windows::ffi::OsStringExt};

use windows::{
    core::PWSTR,
    Win32::{
        Foundation::MAX_PATH,
        Security::Authentication::Identity::{GetUserNameExW, NameSamCompatible},
        System::{
            ProcessStatus::{GetPerformanceInfo, PERFORMANCE_INFORMATION},
            SystemInformation::{
                ComputerNameDnsFullyQualified, GetComputerNameExW, GetSystemDirectoryW,
                GetSystemInfo, SYSTEM_INFO,
            },
            WindowsProgramming::{GetComputerNameW, GetUserNameW},
        },
    },
};

fn print_system_info() {
    println!("1. System Information");

    let system_info: std::mem::MaybeUninit<SYSTEM_INFO> = std::mem::MaybeUninit::uninit();
    unsafe {
        let mut system_info = system_info.assume_init();
        GetSystemInfo(&mut system_info);
        println!(
            " - dwOemId                     : {}",
            system_info.Anonymous.dwOemId
        );
        println!(
            " - dwPageSize                  : {}",
            system_info.dwPageSize
        );
        println!(
            " - lpMinimumApplicationAddress : {:p}",
            system_info.lpMinimumApplicationAddress
        );
        println!(
            " - lpMaximumApplicationAddress : {:p}",
            system_info.lpMaximumApplicationAddress
        );
        println!(
            " - dwActiveProcessorMask       : {}",
            system_info.dwActiveProcessorMask
        );
        println!(
            " - dwNumberOfProcessors        : {}",
            system_info.dwNumberOfProcessors
        );
        println!(
            " - dwProcessorType             : {}",
            system_info.dwProcessorType
        );
        println!(
            " - dwAllocationGranularity     : {}",
            system_info.dwAllocationGranularity
        );
        println!(
            " - wProcessorLevel             : {}",
            system_info.wProcessorLevel
        );
        println!(
            " - wProcessorRevision          : {}\n",
            system_info.wProcessorRevision
        );
    }
}

fn print_computer_name() {
    println!("2. Computer Name");

    let mut size = 32;
    let mut computer_name: Vec<u16> = vec![0; size as usize];
    unsafe {
        GetComputerNameW(PWSTR(computer_name.as_mut_ptr()), &mut size).unwrap();
    };

    let computer_name = OsString::from_wide(&computer_name)
        .to_string_lossy()
        .into_owned();

    println!(" - lpBuffer : {}", computer_name);

    let mut size = 32;
    let mut computer_name: Vec<u16> = vec![0; size as usize];
    unsafe {
        GetComputerNameExW(
            ComputerNameDnsFullyQualified,
            PWSTR(computer_name.as_mut_ptr()),
            &mut size,
        )
        .unwrap();
    };

    let computer_name = OsString::from_wide(&computer_name)
        .to_string_lossy()
        .into_owned();

    println!(" - lpBuffer : {}\n", computer_name);
}

fn print_system_directory() {
    println!("3. System Directory");

    let mut system_directory_path: Vec<u16> = vec![0; MAX_PATH as usize];
    unsafe {
        GetSystemDirectoryW(Some(&mut system_directory_path));
    }

    let system_directory_path = OsString::from_wide(&system_directory_path)
        .to_string_lossy()
        .into_owned();

    println!(" - lpBuffer : {}\n", system_directory_path);
}

fn print_username() {
    println!("4. User Name");

    let mut size = 64;
    let mut username: Vec<u16> = vec![0; size as usize];
    unsafe {
        GetUserNameW(PWSTR(username.as_mut_ptr()), &mut size).unwrap();
    }
    let username = OsString::from_wide(&username)
        .to_string_lossy()
        .into_owned();

    println!(" - lpBuffer : {}", username);

    let mut size = 64;
    let mut username: Vec<u16> = vec![0; size as usize];
    unsafe {
        GetUserNameExW(NameSamCompatible, PWSTR(username.as_mut_ptr()), &mut size);
    }
    let username = OsString::from_wide(&username)
        .to_string_lossy()
        .into_owned();

    println!(" - lpBuffer : {}\n", username);
}

// INFO: PERFORMANCE_INFORMATION has traits which are Default and Debug.
fn print_performance_info() {
    println!("5. Performance Information");

    let mut performance_info: PERFORMANCE_INFORMATION = Default::default();
    unsafe {
        GetPerformanceInfo(
            &mut performance_info,
            std::mem::size_of::<PERFORMANCE_INFORMATION>() as u32,
        )
        .unwrap();

        println!("{:#?}\n", performance_info);
    }
}

// TODO: Call the following functions:
// GetSystemInfo, GetComputerName, GetComputerNameEx, GetSystemDirectory, GetUserName, GetUserNameEx, GetPerformanceInfo
fn main() {
    // GetSystemInfo
    print_system_info();

    // GetComputerName, GetComputerNameEx
    print_computer_name();

    // GetSystemDirectory
    print_system_directory();

    // GetUserName, GetUserNameEx
    print_username();

    // GetPerformanceInfo
    print_performance_info();
}
