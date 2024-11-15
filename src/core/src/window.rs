use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::WindowAttributes,
};

fn initialize_window(event_loop: &EventLoop<()>) -> Result<(), Box<dyn std::error::Error>> {
    // Задаем параметры окна
    let window_attributes = WindowAttributes::default()
        .with_title("My Winit Window")
        .with_inner_size(LogicalSize::new(800.0, 600.0));

    // Создаем окно с заданными атрибутами
    let window = event_loop.create_window(window_attributes)?;

    println!("Окно успешно создано с ID: {:?}", window.id());
    Ok(())
}

pub fn main() {
    // Создаем цикл событий
    let event_loop = EventLoop::new().expect("Не удалось создать EventLoop");

    // Инициализируем окно
    if let Err(e) = initialize_window(&event_loop) {
        eprintln!("Ошибка при инициализации окна: {}", e);
        return;
    }

    // Запускаем цикл обработки событий
    event_loop.run(move |event, _, control_flow| {
        println!("{:?}", event);
        *control_flow = winit::event_loop::ControlFlow::Wait;
    }).expect("TODO: panic message");
}
