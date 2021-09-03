use std::sync::Arc;

use ruma_events::EventType;
use ruma_state_res::{
    event_auth::valid_membership_change,
    test_utils::{alice, charlie, event_id, member_content_ban, to_pdu_event, INITIAL_EVENTS},
    StateMap,
};

#[test]
fn test_ban_pass() {
    let _ = tracing::subscriber::set_default(tracing_subscriber::fmt().with_test_writer().finish());
    let events = INITIAL_EVENTS();

    let prev = events.values().find(|ev| ev.event_id().as_str().contains("IMC")).map(Arc::clone);

    let auth_events = events
        .values()
        .map(|ev| ((ev.event_type(), ev.state_key()), Arc::clone(ev)))
        .collect::<StateMap<_>>();

    let requester = to_pdu_event(
        "HELLO",
        alice(),
        EventType::RoomMember,
        Some(charlie().as_str()),
        member_content_ban(),
        &[],
        &[event_id("IMC")],
    );

    assert!(valid_membership_change(
        &requester.state_key(),
        requester.sender(),
        requester.content(),
        prev,
        None,
        |ty, key| auth_events.get(&(ty.clone(), key.to_owned())).cloned(),
    )
    .unwrap())
}

#[test]
fn test_ban_fail() {
    let _ = tracing::subscriber::set_default(tracing_subscriber::fmt().with_test_writer().finish());
    let events = INITIAL_EVENTS();

    let prev = events.values().find(|ev| ev.event_id().as_str().contains("IMC")).map(Arc::clone);

    let auth_events = events
        .values()
        .map(|ev| ((ev.event_type(), ev.state_key()), Arc::clone(ev)))
        .collect::<StateMap<_>>();

    let requester = to_pdu_event(
        "HELLO",
        charlie(),
        EventType::RoomMember,
        Some(alice().as_str()),
        member_content_ban(),
        &[],
        &[event_id("IMC")],
    );

    assert!(!valid_membership_change(
        &requester.state_key(),
        requester.sender(),
        requester.content(),
        prev,
        None,
        |ty, key| auth_events.get(&(ty.clone(), key.to_owned())).cloned(),
    )
    .unwrap())
}
