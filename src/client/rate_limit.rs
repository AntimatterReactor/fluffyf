// Copyright 2022 @playfulkittykat, @nasso
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License
//
//==========================================================================
//
// Changes are stated below:
// 
// 1. Unsplintered `rate limit` and `dummy rate limit`
// 2. Make the true rate limit optional such that the dummy is not replaced per-se
// 3. Make the choice of rate limiter truly optional to the user
// 4. Added a type alias of DefaultRateLimit
// 
// and are copyrighted as follows:
//
// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A async block wrapper to limit request
//! 
//! # Rate Limit
//!
//! As per [the official wiki](https://e926.net/wiki_pages/2425#api):
//!
//! > E621/E926 have a hard rate limit of two requests per second.
//! > This is a hard upper limit, and if you are hitting it, you are already going way too fast.
//! > Hitting the rate limit will result in a 503 HTTP response code.
//! >
//! > > **Important!**
//! > >
//! > > You should make a best effort not to make more than one request per second over a sustained period.
//! 
//! And so (if you have enabled the `rate-limiter` feature), fluffyf through [`Client`]
//! will delay each request to 600ms per request
//! 
//! Note: This module is not only directly inspired, but quite literally taken from [rs621].
//! See the top of the source file or [the README] for more details and copyright attribution
//! 
//! [`Client`]: crate::client::Client
//! [rs621]: https://github.com/nasso/rs621
//! [the README]: https://github.com/AntimatterReactor/fluffyf/blob/main/README.md

use {
    std::sync::Arc,

    futures::Future,
};

#[cfg(feature = "rate-limit")]
use tokio::{
    sync::{Mutex, MutexGuard},
    time::{sleep_until, Duration, Instant},
};

const REQ_COOLDOWN_DURATION: Duration = Duration::from_millis(600);

pub trait Limiter {
    async fn check<F, R>(self, fut: F) -> R
    where
        F: Future<Output = R>,
        Self: Sized;
}

#[cfg(feature = "rate-limit")]
#[derive(Debug, Clone, Default)]
pub struct RateLimit {
    // Use a tokio mutex for fairness and because ~500ms is crazy long to block
    // an async task.
    deadline: Arc<Mutex<Option<Instant>>>,
}
 
#[cfg(feature = "rate-limit")]
struct Guard<'a>(MutexGuard<'a, Option<Instant>>);

#[cfg(feature = "rate-limit")]
impl<'a> Drop for Guard<'a> {
    fn drop(&mut self) {
        // Use a `Drop` impl so that updating the deadline is panic-safe.
        *self.0 = Some(Instant::now() + REQ_COOLDOWN_DURATION);
    }
}

#[cfg(feature = "rate-limit")]
impl Limiter for RateLimit {
    async fn check<F, R>(self, fut: F) -> R
    where
        F: Future<Output = R>,
    {
        let guard = self.lock().await;
        let result = fut.await;
        drop(guard);
        result
    }
}

#[cfg(feature = "rate-limit")]
impl RateLimit {
    async fn lock(&self) -> Guard {
        loop {
            let now = Instant::now();

            let deadline = {
                let guard = self.deadline.lock().await;

                match &*guard {
                    None => return Guard(guard),
                    Some(deadline) if now >= *deadline => return Guard(guard),
                    Some(deadline) => *deadline,
                }
            };

            sleep_until(deadline).await;
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DummyRateLimit {}

impl Limiter for DummyRateLimit {
    async fn check<F, R>(self, fut: F) -> R
    where
        F: Future<Output = R>,
        Self: Sized
    {
        fut.await
    }
}

#[cfg(feature = "rate-limit")]
pub type DefaultRateLimit = RateLimit;

#[cfg(not(feature = "rate-limit"))]
pub type DefaultRateLimit = DummyRateLimit;
