pub mod configuration;
pub mod domain;
// new entry
pub mod email_client;
pub mod routes;
pub mod startup;
pub mod telemetry;

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
