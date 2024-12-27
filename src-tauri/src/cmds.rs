use tauri::{AppHandle, Manager};
use k8s_openapi::api::core::v1::Event;
use k8s_openapi::api::core::v1::Pod;
use k8s_openapi::api::core::v1::Namespace;
use kube::{api::ListParams, Api, Client, ResourceExt};
use kube::api::DynamicObject;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;

use kube::discovery::{Discovery, verbs, Scope};


use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};

type CmdResult<T = ()> = Result<T, String>;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub async fn greet(name: &str) -> CmdResult<String> {
    let client = Client::try_default().await.map_err(|e| e.to_string())?;

    // 创建 Api 对象用于集群中的所有 Namespaces
    let namespaces_api: Api<Namespace> = Api::all(client.clone());

    // 设置列表参数（可根据需要自定义）
    let namespace_list_params = ListParams::default();

    // 列出所有 Namespaces
    for pod in namespaces_api.list(&namespace_list_params).await.map_err(|e| e.to_string())? {
        println!("Found Namespace {:?}", pod.metadata.name.unwrap())
    }

    let pods: Api<Pod> = Api::namespaced(client.clone(), "processgo");

    let list_params = ListParams::default();

    for pod in pods.list(&list_params).await.map_err(|e| e.to_string())? {
        println!("Found Pod {:?}", pod.metadata.name.unwrap())
    }

    let discovery = Discovery::new(client.clone()).run().await.map_err(|e| e.to_string())?;
    for group in discovery.groups() {
        for (ar, caps) in group.recommended_resources() {
            if !caps.supports_operation(verbs::LIST) {
                continue;
            }
            let api: Api<DynamicObject> = Api::all_with(client.clone(), &ar);
            // can now api.list() to emulate kubectl get all --all
            for obj in api.list(&Default::default()).await.map_err(|e| e.to_string())? {
                println!("{} {}: {}", ar.api_version, ar.kind, obj.name_any());
            }
        }
    }

        // 创建 Api 对象用于集群中的所有 Namespaces
        let crd_api: Api<CustomResourceDefinition> = Api::all(client.clone());

        // 设置列表参数（可根据需要自定义）
        let crd_list_params = ListParams::default();
    
        // 列出所有 Namespaces
        for pod in crd_api.list(&crd_list_params).await.map_err(|e| e.to_string())? {
            println!("Found CRD {:?}", pod.metadata.name.unwrap())
        }

    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

#[tauri::command]
pub async fn close_splashscreen(app: AppHandle) {
    // Show main window
    app.get_webview_window("main").unwrap().show().unwrap();
    // Close splashscreen
    app.get_webview_window("splashscreen")
        .unwrap()
        .close()
        .unwrap();
}

// 定义 Event 信息结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventInfo {
    pub name: String,
    pub namespace: String,
    pub reason: String,
    pub message: String,
    pub involved_object_kind: String,
    pub involved_object_name: String,
    pub first_timestamp: Option<String>,
    pub last_timestamp: Option<String>,
    pub count: u32,
}

#[tauri::command]
pub async fn list_events() -> Result<Vec<EventInfo>, String> {
    // 创建 Kubernetes 客户端
    let client = Client::try_default()
        .await
        .map_err(|e: kube::Error| format!("Failed to create Kubernetes client: {}", e))?;

    // 创建 Api 对象用于所有 Namespaces 中的 Events
    let events_api: Api<Event> = Api::all(client.clone());

    // 设置列表参数，按时间排序（可根据需要自定义）
    let list_params = ListParams::default()
        .timeout(30) // 设置超时时间
        .limit(100); // 设置返回的最大数量

    // 列出所有 Events
    let event_list = events_api.list(&list_params)
        .await
        .map_err(|e| format!("Failed to list events: {}", e))?;

    // 提取 Event 信息
    let events: Vec<EventInfo> = event_list.iter().map(|e| EventInfo {
        name: e.metadata.name.clone().unwrap_or_default(),
        namespace: e.metadata.namespace.clone().unwrap_or_default(),
        reason: e.reason.clone().unwrap_or_default(),
        message: e.message.clone().unwrap_or_default(),
        involved_object_kind: e.involved_object.kind.clone().unwrap_or_default(),
        involved_object_name: e.involved_object.name.clone().unwrap_or_default(),
        first_timestamp: e.first_timestamp.as_ref().map(|t| format!("{:?}", t)),
        last_timestamp: e.last_timestamp.as_ref().map(|t| format!("{:?}", t)),
        count: e.count.unwrap_or(0) as u32,
    }).collect();

    Ok(events)
}