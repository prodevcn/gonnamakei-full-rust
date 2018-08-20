use std::sync::Arc;

use crate::config::InitServiceConfig;
use crate::server::AuthClaims;

/// Carries with all the info of a request.
pub struct RequestContext<C, R> {
    pub request: R,
    pub app_context: C,
    pub config: Arc<InitServiceConfig>,
}

impl<C, R> RequestContext<C, R> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(request: R, app_context: C, config: Arc<InitServiceConfig>) -> Self {
        RequestContext {
            request,
            app_context,
            config,
        }
    }
}

impl<C> RequestContext<C, ()> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new_empty(app_context: C, config: Arc<InitServiceConfig>) -> RequestContext<C, ()> {
        RequestContext {
            request: (),
            app_context,
            config,
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Carries with all the info of a request plus the claims of a JWT token.
pub struct RequestContextWithAuth<C, R> {
    pub request: R,
    pub app_context: C,
    pub config: Arc<InitServiceConfig>,
    pub claims: AuthClaims,
}

impl<C, R> RequestContextWithAuth<C, R> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(context: RequestContext<C, R>, claims: AuthClaims) -> Self {
        RequestContextWithAuth {
            request: context.request,
            app_context: context.app_context,
            config: context.config,
            claims,
        }
    }
}
