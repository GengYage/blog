use gloo::net::http::{Method, Request};
use serde::Deserialize;

pub async fn fetch<T: for<'a> Deserialize<'a>>(
    url: String,
    method: Method,
    body: Option<String>,
) -> Result<T, String> {
    let request = Request::new(&url).method(method);

    let resp = if let Some(body) = body {
        request.body(body)
    } else {
        request
    }
    .send()
    .await;

    match resp {
        Ok(r) if r.status().to_string().starts_with('2') => match r.json::<T>().await {
            Ok(r) => Ok(r),
            Err(e) => Err(format!("无解析响应:{e}")),
        },
        // 错误信息
        Ok(r) => Err(r.text().await.unwrap()),
        Err(e) => Err(format!("请求数据失败:{e}")),
    }
}
