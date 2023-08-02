use crate::utils::{
    service_names::{get_full_service_name, is_full_name},
    systemd::ManagerProxy,
};

/// Stops a service
///
/// TODO support stopping all services with `all`
///
/// # Arguments
///
/// * `name`- Name of the service to stop in short form (hello-world) or long form (hello-world.stabled.service).
///
pub async fn handle_stop_service(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let full_service_name = if is_full_name(&name) {
        name.clone()
    } else {
        get_full_service_name(&name)
    };

    let connection = zbus::Connection::system().await.unwrap();
    let manager_proxy = ManagerProxy::new(&connection).await.unwrap();
    stop_service(&manager_proxy, &full_service_name).await;

    println!("Stopped {name}");

    // handle_show_status().unwrap();

    Ok(())
}

async fn stop_service(manager_proxy: &ManagerProxy<'_>, full_service_name: &String) {
    manager_proxy
        .stop_unit(full_service_name.to_string(), "replace".into())
        .await
        .expect(&format!("Failed to stop service {full_service_name}"));
}
