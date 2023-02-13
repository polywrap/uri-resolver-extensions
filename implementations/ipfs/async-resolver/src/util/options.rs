use crate::wrap::{Env};

pub struct Options<'t> {
    pub disable_parallel_requests: bool,
    pub timeout: u32,
    pub providers: Vec<&'t str>,
}

pub fn get_options<'t>(env: &'t Env) -> Options<'t> {

    let disable_parallel_requests = env.disable_parallel_requests.unwrap_or(false);

    let timeout = env.timeout.unwrap_or(5000);

    let mut providers: Vec<&'t str> = Vec::new();

    providers.push(env.provider.as_ref());

    // env.fallback_providers are added last
    if let Some(fallback_providers) = &env.fallback_providers {
        fallback_providers.iter()
            .map(|s| s.as_ref())
            .for_each(|p| providers.push(p));
    }

    Options {
        disable_parallel_requests,
        timeout,
        providers,
    }
}
