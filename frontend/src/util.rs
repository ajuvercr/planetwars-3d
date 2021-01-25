use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

pub async fn fetch(url: &str) -> Result<String, JsValue> {
    use web_sys::{Request, RequestInit, RequestMode, Response};

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    if !resp.ok() {
        return Err(resp.into());
    }

    // Convert this other `Promise` into a rust `Future`.
    let text = JsFuture::from(resp.text()?).await?.as_string().unwrap();

    Ok(text)
}

pub struct FpsCounter {
    fps: u32,
    time: f64,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self { fps: 0, time: 0.0 }
    }

    pub fn update(&mut self, dt: f64) {
        self.time += dt;
        self.fps += 1;

        if self.time > 1.0 {
            self.time = 0.0;
            console_log!("Fps {}", self.fps);
            self.fps = 0;
        }
    }
}
