
// #include <map>
use std::collections::HashMap;
// #include <set>
use std::collections::HashSet;

// TODO:
//  Port module.h to mod module
// class Module; => mod module;

/*
 * An Event is a lightweight notification of some activity in the BESS core.
 * Currently these are only "sent" to Modules via `Module::OnEvent()`, but they
 * could easily be extended to other entities in the system. See below for a
 * description of
 *
 * PreResume
 * ---------
 * Modules will receive the PreResume event immediately before a call to
 * `resume_worker()` or `resume_all_workers()`. If a Module is attached to
 * multiple workers which are being resumed at the same time (e.g., via
 * WorkerPauser) it will recieve PreResume exactly once.
 */
// enum class Event { PreResume };
enum Event { 
    PreResume,
}

// extern std::map<Event, std::set<Module *>> event_modules;
static event_modules: HashMap<Event, HashSet<Module>> = HashSet::new();
