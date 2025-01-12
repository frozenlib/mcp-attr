use jsoncall::{Hook, RequestId, SessionContext};

use crate::schema::CancelledNotificationParams;

pub(crate) struct McpCancellationHook;
impl Hook for McpCancellationHook {
    fn cancel_outgoing_request(&self, id: RequestId, session: &SessionContext) {
        session
            .notification(
                "notifications/cancelled",
                Some(&CancelledNotificationParams {
                    request_id: id,
                    reason: None,
                }),
            )
            .unwrap()
    }
}
