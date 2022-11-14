// #include <glog/logging.h>

// #include <cstdint>
// #include <string>
// #include <thread>
// #include <type_traits>

// #include "gate.h"
// #include "traffic_class.h"
// #include "utils/common.h"
// #include "utils/random.h"

const MAX_GATES: u32 = 8192;

// /*  TODO: worker threads doesn't necessarily be pinned to 1 core
//  *
//  *  n: kMaxWorkers
//  *
//  *  Role              DPDK lcore ID      Hardware core(s)
//  *  --------------------------------------------------------
//  *  worker 0                      0      1 specified core
//  *  worker 1                      1      1 specified core
//  *  ...
//  *  worker n-1                  n-1      1 specified core
//  *  master          RTE_MAX_LCORE-1      all other cores that are allowed
//  */
pub enum WorkerStatus {
    WorkerPausing, // transient state for blocking or quitting
    WorkerPaused,
    WorkerRunning,
    WorkerFinished,
}

use crate::core::scheduler::Scheduler;
// use crate::packet_pool::PacketPool;

// class Task;
pub const K_MAX_WORKERS: u32 = 64;
pub const K_ANY_WORKER: i32 = -1; // unspecified worker ID

pub struct Worker {
    status: WorkerStatus,
    wid: u32,  // always [0, K_MAX_WORKERS - 1]
    core: u32, // TODO: should be cpuset_t
    socket: u32,
    fd_event: u32,

    // bess::PacketPool *packet_pool_;
    // packet_pool: *PacketPool,
    // scheduler: *Scheduler,

    // bess::Scheduler *scheduler_;
    silent_drops: u64, // packets that have been sent to a deadend
    current_tsc: u64,
    current_ns: u64,
    // rand: &Random,
}

impl Worker {
    //  /* ----------------------------------------------------------------------
    //  * functions below are invoked by non-worker threads (the master)
    //  * ---------------------------------------------------------------------- */
    //  void SetNonWorker();

    //  /* ----------------------------------------------------------------------
    //   * functions below are invoked by worker threads
    //   * ---------------------------------------------------------------------- */
    //  inline int is_pause_requested() { return status_ == WORKER_PAUSING; }

    //  /* Block myself. Return nonzero if the worker needs to die */
    //  int BlockWorker();

    //  /* The entry point of worker threads */
    //  void *Run(void *_arg);

    //  worker_status_t status() { return status_; }
    //  void set_status(worker_status_t status) { status_ = status; }

    //  int wid() { return wid_; }
    //  int core() { return core_; }
    //  int socket() { return socket_; }
    //  int fd_event() { return fd_event_; }

    //  bess::PacketPool *packet_pool() { return packet_pool_; }

    //  bess::Scheduler *scheduler() { return scheduler_; }

    //  uint64_t silent_drops() { return silent_drops_; }
    //  void set_silent_drops(uint64_t drops) { silent_drops_ = drops; }
    //  void incr_silent_drops(uint64_t drops) { silent_drops_ += drops; }

    //  uint64_t current_tsc() const { return current_tsc_; }
    //  void set_current_tsc(uint64_t tsc) { current_tsc_ = tsc; }

    //  uint64_t current_ns() const { return current_ns_; }
    //  void set_current_ns(uint64_t ns) { current_ns_ = ns; }

    //  Random *rand() const { return rand_; }
}
// NOTE: Do not use "thread_local" here. It requires a function call every time
// it is accessed. Use __thread instead, which incurs minimal runtime overhead.
// For this, g++ requires Worker to have a trivial constructor and destructor.
// extern __thread Worker current_worker;

// // the following traits are not supported in g++ 4.x
// #if __GNUC__ >= 5
// static_assert(std::is_trivially_constructible<Worker>::value,
//               "not trivially constructible");
// static_assert(std::is_trivially_destructible<Worker>::value,
//               "not trivially destructible");
// #endif

// // TODO: C++-ify

// extern int num_workers;
// extern std::thread worker_threads[Worker::kMaxWorkers];
// extern Worker *volatile workers[Worker::kMaxWorkers];

// /* ------------------------------------------------------------------------
//  * functions below are invoked by non-worker threads (the master)
//  * ------------------------------------------------------------------------ */
// int is_worker_core(int cpu);

// void pause_worker(int wid);
// void pause_all_workers();

// /*!
//  * Attach orphan TCs to workers. Note this does not ensure optimal placement.
//  */
// void attach_orphans();
// void resume_worker(int wid);
// void resume_all_workers();
// void destroy_worker(int wid);
// void destroy_all_workers();

// bool is_any_worker_running();

// int is_cpu_present(unsigned int core_id);

//TODO
// static inline int is_worker_active(int wid) {
//   return workers[wid] != nullptr;
// }
pub fn is_worker_active(wid: i64) -> bool {
    true
}

// inline bool is_worker_running(int wid) {
//   return workers[wid] && workers[wid]->status() == WORKER_RUNNING;
// }

// // arg (int) is the core id the worker should run on, and optionally the
// // scheduler to use.
// void launch_worker(int wid, int core, const std::string &scheduler = "");

// Worker *get_next_active_worker();

// // Add 'c' to the list of orphan traffic classes.
// void add_tc_to_orphan(bess::TrafficClass *c, int wid);

// // Return true if 'c' was removed from the list of orphan traffic classes.
// // 'c' is now owned by the caller, and it must be attached to a tree or
// // destroyed.
// //
// // Otherwise, return false
// bool remove_tc_from_orphan(bess::TrafficClass *c);

// // Returns a list of all the orphan traffic classes.
// const std::list<std::pair<int, bess::TrafficClass *>> &list_orphan_tcs();

// // Try to detach 'c' from a scheduler, or from the list of orhpan traffic
// // classes.
// //
// // Return true if successful. 'c' is now owned by the caller, and it must be
// // attached to a tree or destroyed.
// //
// // Otherwise, return false
// bool detach_tc(bess::TrafficClass *c);

// // This class is used as a resource manager to automatically pause workers if
// // running and then restarts workers if they were previously paused.
// class WorkerPauser {
//  public:
//   explicit WorkerPauser();
//   ~WorkerPauser();

//  private:
//   std::list<int> workers_paused_;
// };

// #endif  // BESS_WORKER_H_
// ===========================================================================================

// #include <sched.h>
// #include <sys/eventfd.h>
// #include <unistd.h>

// #include <glog/logging.h>
// #include <rte_config.h>
// #include <rte_lcore.h>

// #include <cassert>
// #include <climits>
// #include <list>
// #include <string>
// #include <utility>

// #include "metadata.h"
// #include "module.h"
// #include "opts.h"
// #include "packet_pool.h"
// #include "resume_hook.h"
// #include "resume_hooks/metadata.h"
// #include "scheduler.h"
// #include "utils/random.h"
// #include "utils/time.h"

// using bess::DefaultScheduler;
// using bess::ExperimentalScheduler;
// using bess::Scheduler;

// int num_workers = 0;
// std::thread worker_threads[Worker::kMaxWorkers];
// Worker *volatile workers[Worker::kMaxWorkers];

// using bess::TrafficClassBuilder;
// using namespace bess::traffic_class_initializer_types;
// using bess::ResumeHookBuilder;

// std::list<std::pair<int, bess::TrafficClass *>> orphan_tcs;

// // See worker.h
// __thread Worker current_worker;

// struct thread_arg {
//   int wid;
//   int core;
//   Scheduler *scheduler;
// };

// #define SYS_CPU_DIR "/sys/devices/system/cpu/cpu%u"
// #define CORE_ID_FILE "topology/core_id"

// Check if a cpu is present by the presence of the cpu information for it
// pub fn is_cpu_present(core_id: u32) -> bool {
//   char path[PATH_MAX];
//   int len = snprintf(path, sizeof(path), SYS_CPU_DIR "/" CORE_ID_FILE, core_id);
//   if (len <= 0 || (unsigned)len >= sizeof(path)) {
//     return false;
//   }
//   if (access(path, F_OK) != 0) {
//     return false;
//   }

// return true;
// }

// int is_worker_core(int cpu) {
//   int wid;

//   for (wid = 0; wid < Worker::kMaxWorkers; wid++) {
//     if (is_worker_active(wid) && workers[wid]->core() == cpu)
//       return 1;
//   }

//   return 0;
// }

// void pause_worker(int wid) {
//   if (workers[wid] && workers[wid]->status() == WORKER_RUNNING) {
//     workers[wid]->set_status(WORKER_PAUSING);

//     FULL_BARRIER();

//     while (workers[wid]->status() == WORKER_PAUSING) {
//     } /* spin */
//   }
// }

// void pause_all_workers() {
//   for (int wid = 0; wid < Worker::kMaxWorkers; wid++)
//     pause_worker(wid);
// }

// enum class worker_signal : uint64_t {
//   unblock = 1,
//   quit,
// };

// void resume_worker(int wid) {
//   if (workers[wid] && workers[wid]->status() == WORKER_PAUSED) {
//     int ret;
//     worker_signal sig = worker_signal::unblock;

//     ret = write(workers[wid]->fd_event(), &sig, sizeof(sig));
//     CHECK_EQ(ret, sizeof(uint64_t));

//     while (workers[wid]->status() == WORKER_PAUSED) {
//     } /* spin */
//   }
// }

// /*!
//  * Attach orphan TCs to workers. Note this does not ensure optimal placement.
//  * This method can only be called when all workers are paused.
//  */
// void attach_orphans() {
//   CHECK(!is_any_worker_running());
//   // Distribute all orphan TCs to workers.
//   for (const auto &tc : orphan_tcs) {
//     bess::TrafficClass *c = tc.second;
//     if (c->parent()) {
//       continue;
//     }

//     Worker *w;

//     int wid = tc.first;
//     if (wid == Worker::kAnyWorker || workers[wid] == nullptr) {
//       w = get_next_active_worker();
//     } else {
//       w = workers[wid];
//     }

//     w->scheduler()->AttachOrphan(c, w->wid());
//   }

//   orphan_tcs.clear();
// }

// void resume_all_workers() {
//   for (int wid = 0; wid < Worker::kMaxWorkers; wid++) {
//     if (workers[wid]) {
//       workers[wid]->scheduler()->AdjustDefault();
//     }
//   }

//   for (int wid = 0; wid < Worker::kMaxWorkers; wid++) {
//     resume_worker(wid);
//   }
// }

// void destroy_worker(int wid) {
//   pause_worker(wid);

//   if (workers[wid] && workers[wid]->status() == WORKER_PAUSED) {
//     int ret;
//     worker_signal sig = worker_signal::quit;

//     ret = write(workers[wid]->fd_event(), &sig, sizeof(sig));
//     CHECK_EQ(ret, sizeof(uint64_t));

//     while (workers[wid]->status() == WORKER_PAUSED) {
//     } /* spin */
//     workers[wid] = nullptr;

//     num_workers--;
//   }

//   if (num_workers > 0) {
//     return;
//   }

//   auto &hooks = bess::global_resume_hooks;
//   for (auto it = hooks.begin(); it != hooks.end();) {
//     if ((*it)->is_default()) {
//       it++;
//     } else {
//       it = hooks.erase(it);
//     }
//   }
// }

// void destroy_all_workers() {
//   for (int wid = 0; wid < Worker::kMaxWorkers; wid++) {
//     destroy_worker(wid);
//   }
// }

// bool is_any_worker_running() {
//   int wid;

//   for (wid = 0; wid < Worker::kMaxWorkers; wid++) {
//     if (is_worker_running(wid)) {
//       return true;
//     }
//   }

//   return false;
// }

// void Worker::SetNonWorker() {
//   // These TLS variables should not be accessed by non-worker threads.
//   // Assign INT_MIN to the variables so that the program can crash
//   // when accessed as an index of an array.
//   wid_ = INT_MIN;
//   core_ = INT_MIN;
//   socket_ = INT_MIN;
//   fd_event_ = INT_MIN;

//   if (!packet_pool_) {
//     // Packet pools should be available to non-worker threads.
//     // (doesn't need to be NUMA-aware, so pick any)
//     for (int socket = 0; socket < RTE_MAX_NUMA_NODES; socket++) {
//       if (bess::PacketPool *pool = bess::PacketPool::GetDefaultPool(socket)) {
//         packet_pool_ = pool;
//         break;
//       }
//     }
//   }
// }

// int Worker::BlockWorker() {
//   worker_signal t;
//   int ret;

//   status_ = WORKER_PAUSED;

//   ret = read(fd_event_, &t, sizeof(t));
//   CHECK_EQ(ret, sizeof(t));

//   if (t == worker_signal::unblock) {
//     status_ = WORKER_RUNNING;
//     return 0;
//   }

//   if (t == worker_signal::quit) {
//     status_ = WORKER_FINISHED;
//     return 1;
//   }

//   CHECK(0);
//   return 0;
// }

// /* The entry point of worker threads */
// void *Worker::Run(void *_arg) {
//   struct thread_arg *arg = (struct thread_arg *)_arg;
//   rand_ = new Random();

//   cpu_set_t set;

//   CPU_ZERO(&set);
//   CPU_SET(arg->core, &set);
//   rte_thread_set_affinity(&set);

//   /* DPDK lcore ID == worker ID (0, 1, 2, 3, ...) */
//   RTE_PER_LCORE(_lcore_id) = arg->wid;

//   /* for workers, wid == rte_lcore_id() */
//   wid_ = arg->wid;
//   core_ = arg->core;
//   socket_ = rte_socket_id();

//   // For some reason, rte_socket_id() does not return a correct NUMA ID.
//   // Nevertheless, BESS should not crash.
//   if (socket_ == SOCKET_ID_ANY) {
//     LOG(WARNING) << "rte_socket_id() returned -1 for " << arg->core;
//     socket_ = 0;
//   }

//   fd_event_ = eventfd(0, 0);
//   CHECK_GE(fd_event_, 0);

//   scheduler_ = arg->scheduler;

//   current_tsc_ = rdtsc();

//   packet_pool_ = bess::PacketPool::GetDefaultPool(socket_);
//   CHECK_NOTNULL(packet_pool_);

//   status_ = WORKER_PAUSING;

//   STORE_BARRIER();

//   workers[wid_] = this;  // FIXME: consider making workers a static member
//                          // instead of a global

//   LOG(INFO) << "Worker " << wid_ << "(" << this << ") "
//             << "is running on core " << core_ << " (socket " << socket_ << ")";

//   CPU_ZERO(&set);
//   scheduler_->ScheduleLoop();

//   LOG(INFO) << "Worker " << wid_ << "(" << this << ") "
//             << "is quitting... (core " << core_ << ", socket " << socket_
//             << ")";

//   delete scheduler_;
//   delete rand_;

//   return nullptr;
// }

// void *run_worker(void *_arg) {
//   CHECK_EQ(memcmp(&current_worker, new Worker(), sizeof(Worker)), 0);
//   return current_worker.Run(_arg);
// }

// void launch_worker(int wid, int core,
//                    [[maybe_unused]] const std::string &scheduler) {
//   struct thread_arg arg = {.wid = wid, .core = core, .scheduler = nullptr};
//   if (scheduler == "") {
//     arg.scheduler = new DefaultScheduler();
//   } else if (scheduler == "experimental") {
//     arg.scheduler = new ExperimentalScheduler();
//   } else {
//     CHECK(false) << "Scheduler " << scheduler << " is invalid.";
//   }

//   worker_threads[wid] = std::thread(run_worker, &arg);
//   worker_threads[wid].detach();

//   INST_BARRIER();

//   /* spin until it becomes ready and fully paused */
//   while (!is_worker_active(wid) || workers[wid]->status() != WORKER_PAUSED) {
//     continue;
//   }

//   num_workers++;
// }

// Worker *get_next_active_worker() {
//   static int prev_wid = 0;
//   if (num_workers == 0) {
//     launch_worker(0, FLAGS_c);
//     return workers[0];
//   }

//   while (!is_worker_active(prev_wid)) {
//     prev_wid = (prev_wid + 1) % Worker::kMaxWorkers;
//   }

//   Worker *ret = workers[prev_wid];
//   prev_wid = (prev_wid + 1) % Worker::kMaxWorkers;
//   return ret;
// }

// void add_tc_to_orphan(bess::TrafficClass *c, int wid) {
//   orphan_tcs.emplace_back(wid, c);
// }

// bool remove_tc_from_orphan(bess::TrafficClass *c) {
//   for (auto it = orphan_tcs.begin(); it != orphan_tcs.end();) {
//     if (it->second == c) {
//       orphan_tcs.erase(it);
//       return true;
//     } else {
//       it++;
//     }
//   }

//   return false;
// }

// const std::list<std::pair<int, bess::TrafficClass *>> &list_orphan_tcs() {
//   return orphan_tcs;
// }

// bool detach_tc(bess::TrafficClass *c) {
//   bess::TrafficClass *parent = c->parent();
//   if (parent) {
//     return parent->RemoveChild(c);
//   }

//   // Try to remove from root of one of the schedulers
//   for (int wid = 0; wid < Worker::kMaxWorkers; wid++) {
//     if (workers[wid]) {
//       bool found = workers[wid]->scheduler()->RemoveRoot(c);
//       if (found) {
//         return true;
//       }
//     }
//   }

//   // Try to remove from orphan_tcs
//   return remove_tc_from_orphan(c);
// }

// WorkerPauser::WorkerPauser() {
//   if (is_any_worker_running()) {
//     for (int wid = 0; wid < Worker::kMaxWorkers; wid++) {
//       if (is_worker_running(wid)) {
//         workers_paused_.push_back(wid);
//         VLOG(1) << "*** Pausing Worker " << wid << " ***";
//         pause_worker(wid);
//       }
//     }
//   }
// }

// WorkerPauser::~WorkerPauser() {
//   attach_orphans();  // All workers should be paused at this point.

//   if (!workers_paused_.empty()) {
//     bess::run_global_resume_hooks(false);
//   }

//   std::set<Module *> modules_run;
//   for (int wid : workers_paused_) {
//     auto &resume_modules = bess::event_modules[bess::Event::PreResume];
//     for (auto it = resume_modules.begin(); it != resume_modules.end();) {
//       Module *m = *it;
//       if (!modules_run.count(m) && m->active_workers()[wid]) {
//         int ret = m->OnEvent(bess::Event::PreResume);
//         modules_run.insert(m);
//         if (ret == -ENOTSUP) {
//           it = resume_modules.erase(it);
//         } else {
//           it++;
//         }
//       } else {
//         it++;
//       }
//     }
//     resume_worker(wid);
//     VLOG(1) << "*** Worker " << wid << " Resumed ***";
//   }
// }
