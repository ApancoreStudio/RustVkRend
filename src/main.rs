#![allow(unused)]
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::VulkanLibrary;

use vulkano::device::QueueFlags;
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo};

use std::sync::Arc;
use vulkano::memory::allocator::StandardMemoryAllocator;

fn main() {
    //инициализация библиотеки
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance =
        Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");
    //выбор физического девайса
    let physical_device = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices")
        .next()
        .expect("no devices available");
    //обработка и создание очередей на физическом устройстве
    for family in physical_device.queue_family_properties() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queue_count
        );
    }
    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties
                .queue_flags
                .contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;
    //создание логического девайса
    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("failed to create device");
    //выбор конкретной очереди для использования в дальнейшем
    let queue = queues.next().unwrap();
    //инициализация распределителя памяти
    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
}
