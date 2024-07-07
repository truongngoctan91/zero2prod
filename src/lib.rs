pub mod configuration;
pub mod routes;
pub mod startup;

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
