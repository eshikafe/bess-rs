// ==================

// #include "task.h"
// #include "utils/common.h"
// #include "utils/extended_priority_queue.h"
// #include "utils/simd.h"
// #include "utils/time.h"

use traffic_class_initializer_types::*;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use lazy_static::lazy_static;

// using bess::utils::extended_priority_queue;

// A large default priority.
const DEFAULT_PRIORITY: u32 = 0xFFFFFFFF;

// Share is defined relatively, so 1024 should be large enough
const STRIDE1: u32 = 1 << 20;

// This doesn't mean anything, other than avoiding int64 overflow
const QUANTUM: u32 = 1 << 10;

// Resource types that can be accounted for.
#[derive(PartialEq, Eq, Hash)]
pub enum Resource {
    Count = 0, // Count of how many times scheduled
    Cycle,     // CPU cycles
    Packet,    // Packets set
    Bit,       // Bits sent
    NumResources,      // Sentinel. Also used to indicate "no resource".
}

// An array of counters for all resource types.
// pub type resource_arr_t = [u64; Resource::NUM_RESOURCES as usize];

// The priority of a traffic class.
type priority_t = u32;

// The amount of a resource allocated to a class.
type resource_share_t = u32;

struct TcStats {
    usage: [u64; 4],
    cnt_throttled: u64,
}

// class Scheduler;
// class SchedWakeupQueue;
// class TrafficClassBuilder;
// class PriorityTrafficClass;
// class WeightedFairTrafficClass;
// class RoundRobinTrafficClass;
// class RateLimitTrafficClass;
// class LeafTrafficClass;
// class TrafficClass;

enum TrafficPolicy {
    PolicyPriority = 0,
    PolicyWeightedFair,
    PolicyRoundRobin,
    PolicyRateLimit,
    PolicyLeaf,
    NumPolicies, // sentinel
}

mod traffic_class_initializer_types {
    enum PriorityFakeType {
        Priority,
    }

    enum WeightedFairFakeType {
        WeightedFair,
    }
    enum RoundRobinFakeType {
        RoundRobin,
    }
    enum RateLimitFakeType {
        RateLimit,
    }
    enum LeafFakeType {
        Leaf,
    }
}

const TRAFFIC_POLICY_NAME: [&str; TrafficPolicy::NumPolicies as usize] = [
    "priority",
    "weighted_fair",
    "round_robin",
    "rate_limit",
    "leaf",
];

static RESOURCE_MAP: Lazy<HashMap<&'static str, Resource>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("count",Resource::Count);
    m.insert("cycle",Resource::Cycle);
    m.insert("packet", Resource::Packet);
    m.insert("bit",Resource::Bit);
    m
});

static RESOURCE_NAME: Lazy<HashMap<Resource, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(Resource::Count, "count");
    m.insert(Resource::Cycle, "cycle");
    m.insert(Resource::Packet, "packet");
    m.insert(Resource::Bit, "bit");
    m
});
/* acc += x */
// #define ACCUMULATE(acc, x)                                \
//   {                                                       \
//     uint64_t *p1 = acc;                                   \
//     uint64_t *p2 = x;                                     \
//     for (int index = 0; index < NUM_RESOURCES; ++index) { \
//       p1[index] += p2[index];                             \
//     }                                                     \
//   }

// class TCChildArgs {
//  public:
//   TCChildArgs(TrafficClass *child)
//       : parent_type_(NUM_POLICIES), child_(child) {}
//   TrafficClass *child() { return child_; }
//   TrafficPolicy parent_type() { return parent_type_; }

//  protected:
//   TCChildArgs(TrafficPolicy parent_type, TrafficClass *child)
//       : parent_type_(parent_type), child_(child) {}

//  private:
//   const TrafficPolicy parent_type_;
//   TrafficClass *child_;
// };

// A TrafficClass represents a hierarchy of TrafficClasses which contain
// schedulable task units.
pub struct TrafficClass {
    // Parent of this class; nullptr for root.
    parent: Box<TrafficClass>,

    // The name given to this class.
    name: String,
    stats: TcStats,

    // The tsc time that this should be woken up by the scheduler.
    wakeup_time: u64,

    // private:
    // friend class Scheduler;
    // friend class DefaultScheduler;
    // friend class ExperimentalScheduler;
    blocked: bool,
    policy: TrafficPolicy,
}

impl TrafficClass {
    //   TrafficClass(const std::string &name, const TrafficPolicy &policy,
    //     bool blocked = true)
    // : parent_(),
    // name_(name),
    // stats_(),
    // wakeup_time_(),
    // blocked_(blocked),
    // policy_(policy) {}

    // pub fn new(name: String, policy: TrafficPolicy, blocked: bool) -> TrafficClass {
    //   TrafficClass { parent: Self, name: name, stats: (), wakeup_time: (), blocked: true, policy: policy }

    // }

    // Returns the number of TCs in the TC subtree rooted at this, including
    // this TC.
    pub fn size() -> usize {
        0
    }

    // use Traits
    //   virtual std::vector<TrafficClass *> Children() const = 0;

    //   // Returns the root of the tree this class belongs to.
    //   // Expensive in that it is recursive, so do not call from
    //   // performance-sensitive code.
    //   const TrafficClass *Root() const { return parent_ ? parent_->Root() : this; }
    //   TrafficClass *Root() { return parent_ ? parent_->Root() : this; }

    //   // Returns its worker ID, or -1 (kAnyWorker) if not belongs to any worker yet
    //   int WorkerId() const;

    //   // Returns true if 'child' was removed successfully, in which case
    //   // the caller owns it. Therefore, after a successful call, 'child'
    //   // must be destroyed or attached to another tree.
    //   virtual bool RemoveChild(TrafficClass *child) = 0;

    //   // Starts from the current node and accounts for the usage of the given child
    //   // after execution and finishes any data structure reorganization required
    //   // after execution has finished.
    //   virtual void FinishAndAccountTowardsRoot(SchedWakeupQueue *wakeup_queue,
    //                                            TrafficClass *child,
    //                                            resource_arr_t usage,
    //                                            uint64_t tsc) = 0;

    //   TrafficClass *parent() const { return parent_; }

    //   const std::string &name() const { return name_; }

    //   const struct tc_stats &stats() const { return stats_; }

    //   uint64_t wakeup_time() const { return wakeup_time_; }

    //   bool blocked() const { return blocked_; }

    //   TrafficPolicy policy() const { return policy_; }

    //  protected:
    //   friend PriorityTrafficClass;
    //   friend WeightedFairTrafficClass;
    //   friend RoundRobinTrafficClass;
    //   friend RateLimitTrafficClass;
    //   friend class LeafTrafficClass;

    //   // Sets blocked status to nowblocked and recurses towards root by signaling
    //   // the parent if status became unblocked.
    //   void UnblockTowardsRootSetBlocked(uint64_t tsc, bool nowblocked) {
    //     bool became_unblocked = !nowblocked && blocked_;
    //     blocked_ = nowblocked;

    //     if (!parent_ || !became_unblocked) {
    //       return;
    //     }

    //     parent_->UnblockTowardsRoot(tsc);
    //   }

    //   // Sets blocked status to nowblocked and recurses towards root by signaling
    //   // the parent if status became blocked.
    //   void BlockTowardsRootSetBlocked(bool nowblocked) {
    //     bool became_blocked = nowblocked && !blocked_;
    //     blocked_ = nowblocked;

    //     if (!parent_ || !became_blocked) {
    //       return;
    //     }

    //     parent_->BlockTowardsRoot();
    //   }

    //   // Returns the next schedulable child of this traffic class.
    //   virtual TrafficClass *PickNextChild() = 0;

    //   // Starts from the current node and attempts to recursively unblock (if
    //   // eligible) all nodes from this node to the root.
    //   virtual void UnblockTowardsRoot(uint64_t tsc) = 0;

    //   // Starts from the current node and attempts to recursively block (if
    //   // eligible) all nodes from this node to the root.
    //   virtual void BlockTowardsRoot() = 0;
}

// class TrafficClass {
//  public:
//   virtual ~TrafficClass() {}

//   // Returns the number of TCs in the TC subtree rooted at this, including
//   // this TC.
//   size_t Size() const;

//   virtual std::vector<TrafficClass *> Children() const = 0;

//   // Returns the root of the tree this class belongs to.
//   // Expensive in that it is recursive, so do not call from
//   // performance-sensitive code.
//   const TrafficClass *Root() const { return parent_ ? parent_->Root() : this; }
//   TrafficClass *Root() { return parent_ ? parent_->Root() : this; }

//   // Returns its worker ID, or -1 (kAnyWorker) if not belongs to any worker yet
//   int WorkerId() const;

//   // Returns true if 'child' was removed successfully, in which case
//   // the caller owns it. Therefore, after a successful call, 'child'
//   // must be destroyed or attached to another tree.
//   virtual bool RemoveChild(TrafficClass *child) = 0;

//   // Starts from the current node and accounts for the usage of the given child
//   // after execution and finishes any data structure reorganization required
//   // after execution has finished.
//   virtual void FinishAndAccountTowardsRoot(SchedWakeupQueue *wakeup_queue,
//                                            TrafficClass *child,
//                                            resource_arr_t usage,
//                                            uint64_t tsc) = 0;

//   TrafficClass *parent() const { return parent_; }

//   const std::string &name() const { return name_; }

//   const struct tc_stats &stats() const { return stats_; }

//   uint64_t wakeup_time() const { return wakeup_time_; }

//   bool blocked() const { return blocked_; }

//   TrafficPolicy policy() const { return policy_; }

//  protected:
//   friend PriorityTrafficClass;
//   friend WeightedFairTrafficClass;
//   friend RoundRobinTrafficClass;
//   friend RateLimitTrafficClass;
//   friend class LeafTrafficClass;

//   TrafficClass(const std::string &name, const TrafficPolicy &policy,
//                bool blocked = true)
//       : parent_(),
//         name_(name),
//         stats_(),
//         wakeup_time_(),
//         blocked_(blocked),
//         policy_(policy) {}

//   // Sets blocked status to nowblocked and recurses towards root by signaling
//   // the parent if status became unblocked.
//   void UnblockTowardsRootSetBlocked(uint64_t tsc, bool nowblocked) {
//     bool became_unblocked = !nowblocked && blocked_;
//     blocked_ = nowblocked;

//     if (!parent_ || !became_unblocked) {
//       return;
//     }

//     parent_->UnblockTowardsRoot(tsc);
//   }

//   // Sets blocked status to nowblocked and recurses towards root by signaling
//   // the parent if status became blocked.
//   void BlockTowardsRootSetBlocked(bool nowblocked) {
//     bool became_blocked = nowblocked && !blocked_;
//     blocked_ = nowblocked;

//     if (!parent_ || !became_blocked) {
//       return;
//     }

//     parent_->BlockTowardsRoot();
//   }

//   // Returns the next schedulable child of this traffic class.
//   virtual TrafficClass *PickNextChild() = 0;

//   // Starts from the current node and attempts to recursively unblock (if
//   // eligible) all nodes from this node to the root.
//   virtual void UnblockTowardsRoot(uint64_t tsc) = 0;

//   // Starts from the current node and attempts to recursively block (if
//   // eligible) all nodes from this node to the root.
//   virtual void BlockTowardsRoot() = 0;

//   // Parent of this class; nullptr for root.
//   TrafficClass *parent_;

//   // The name given to this class.
//   const std::string name_;

//   struct tc_stats stats_;

//   // The tsc time that this should be woken up by the scheduler.
//   uint64_t wakeup_time_;

//  private:
//   friend class Scheduler;
//   friend class DefaultScheduler;
//   friend class ExperimentalScheduler;

//   bool blocked_;

//   const TrafficPolicy policy_;

// DISALLOW_COPY_AND_ASSIGN(TrafficClass);
// };

// class PriorityTrafficClass final : public TrafficClass {
//  public:
//   struct ChildData {
//     bool operator<(const ChildData &right) const {
//       return priority_ < right.priority_;
//     }

//     priority_t priority_;
//     TrafficClass *c_;
//   };

//   explicit PriorityTrafficClass(const std::string &name)
//       : TrafficClass(name, POLICY_PRIORITY), first_runnable_(0), children_() {}

//   ~PriorityTrafficClass();

//   std::vector<TrafficClass *> Children() const override;

//   // Returns true if child was added successfully.
//   bool AddChild(TrafficClass *child, priority_t priority);

//   // Returns true if child was removed successfully.
//   bool RemoveChild(TrafficClass *child) override;

//   TrafficClass *PickNextChild() override;

//   void UnblockTowardsRoot(uint64_t tsc) override;
//   void BlockTowardsRoot() override;

//   void FinishAndAccountTowardsRoot(SchedWakeupQueue *wakeup_queue,
//                                    TrafficClass *child, resource_arr_t usage,
//                                    uint64_t tsc) override;

//   const std::vector<ChildData> &children() const { return children_; }

//  private:
//   size_t
//       first_runnable_;  // Index of first member of children_ that is runnable.
//   std::vector<ChildData> children_;
// };

// class WeightedFairTrafficClass final : public TrafficClass {
//  public:
//   struct ChildData {
//     bool operator<(const ChildData &right) const {
//       // Reversed so that priority_queue is a min priority queue.
//       return right.pass < pass;
//     }

//     double stride;

//     // NOTE: while in the code example in the original Stride Scheduler
//     // [Waldspurgger95] maintains "pass" and "remain" (penalty) separately,
//     // we can safely multiplex these variables in a union since they are never
//     // used at the same time.
//     union {
//       double pass;
//       double remain;
//     };

//     TrafficClass *c;
//   };

//   WeightedFairTrafficClass(const std::string &name, resource_t resource)
//       : TrafficClass(name, POLICY_WEIGHTED_FAIR),
//         resource_(resource),
//         runnable_children_(),
//         blocked_children_(),
//         all_children_() {}

//   ~WeightedFairTrafficClass();

//   std::vector<TrafficClass *> Children() const override;

//   // Returns true if child was added successfully.
//   bool AddChild(TrafficClass *child, resource_share_t share);

//   // Returns true if child was removed successfully.
//   bool RemoveChild(TrafficClass *child) override;

//   TrafficClass *PickNextChild() override;

//   void UnblockTowardsRoot(uint64_t tsc) override;
//   void BlockTowardsRoot() override;

//   void FinishAndAccountTowardsRoot(SchedWakeupQueue *wakeup_queue,
//                                    TrafficClass *child, resource_arr_t usage,
//                                    uint64_t tsc) override;

//   resource_t resource() const { return resource_; }

//   void set_resource(resource_t res) { resource_ = res; }

//   const extended_priority_queue<ChildData> &runnable_children() const {
//     return runnable_children_;
//   }

//   const std::list<ChildData> &blocked_children() const {
//     return blocked_children_;
//   }

//   const std::vector<std::pair<TrafficClass *, resource_share_t>> &children()
//       const {
//     return all_children_;
//   }

//  private:
//   // Returns the pass value of the first child to be scheduled next,
//   // or 0 if there is no runnable child (i.e., the priority queue is empty)
//   double NextPass() const {
//     if (runnable_children_.empty()) {
//       return 0;
//     } else {
//       return runnable_children_.top().pass;
//     }
//   }

//   // The resource that we are sharing.
//   resource_t resource_;

//   extended_priority_queue<ChildData> runnable_children_;
//   std::list<ChildData> blocked_children_;

//   // This is a copy of the pointers to (and shares of) all children. It can be
//   // safely accessed from the master thread while the workers are running.
//   std::vector<std::pair<TrafficClass *, resource_share_t>> all_children_;
// };

// class RoundRobinTrafficClass final : public TrafficClass {
//  public:
//   explicit RoundRobinTrafficClass(const std::string &name)
//       : TrafficClass(name, POLICY_ROUND_ROBIN),
//         next_child_(),
//         runnable_children_(),
//         blocked_children_(),
//         all_children_() {}

//   ~RoundRobinTrafficClass();

//   std::vector<TrafficClass *> Children() const override {
//     return all_children_;
//   }

//   // Returns true if child was added successfully.
//   bool AddChild(TrafficClass *child);

//   // Returns true if child was removed successfully.
//   bool RemoveChild(TrafficClass *child) override;

//   TrafficClass *PickNextChild() override;

//   void UnblockTowardsRoot(uint64_t tsc) override;
//   void BlockTowardsRoot() override;

//   void FinishAndAccountTowardsRoot(SchedWakeupQueue *wakeup_queue,
//                                    TrafficClass *child, resource_arr_t usage,
//                                    uint64_t tsc) override;

//   const std::vector<TrafficClass *> &runnable_children() const {
//     return runnable_children_;
//   }

//   const std::list<TrafficClass *> &blocked_children() const {
//     return blocked_children_;
//   }

//  private:
//   size_t next_child_;

//   std::vector<TrafficClass *> runnable_children_;
//   std::list<TrafficClass *> blocked_children_;

//   // This is a copy of the pointers to all children. It can be safely
//   // accessed from the master thread while the workers are running.
//   std::vector<TrafficClass *> all_children_;
// };

// // Performs rate limiting on a single child class (which could implement some
// // other policy with many children).  Rate limit policy is special, because it
// // can block and because there is a one-to-one parent-child relationship.
// class RateLimitTrafficClass final : public TrafficClass {
//  public:
//   RateLimitTrafficClass(const std::string &name, resource_t resource,
//                         uint64_t limit, uint64_t max_burst)
//       : TrafficClass(name, POLICY_RATE_LIMIT),
//         resource_(resource),
//         limit_(),
//         limit_arg_(),
//         max_burst_(),
//         max_burst_arg_(),
//         tokens_(),
//         last_tsc_(),
//         child_() {
//     set_limit(limit);
//     set_max_burst(max_burst);
//   }

//   ~RateLimitTrafficClass();

//   std::vector<TrafficClass *> Children() const override;

//   // Returns true if child was added successfully.
//   bool AddChild(TrafficClass *child);

//   // Returns true if child was removed successfully.
//   bool RemoveChild(TrafficClass *child) override;

//   TrafficClass *PickNextChild() override;

//   void UnblockTowardsRoot(uint64_t tsc) override;
//   void BlockTowardsRoot() override;

//   void FinishAndAccountTowardsRoot(SchedWakeupQueue *wakeup_queue,
//                                    TrafficClass *child, resource_arr_t usage,
//                                    uint64_t tsc) override;

//   resource_t resource() const { return resource_; }

//   // Return the configured limit, in work units
//   uint64_t limit() const { return limit_; }

//   // Return the configured max burst, in work units
//   uint64_t max_burst() const { return max_burst_; }

//   // Return the configured limit, in resource units
//   uint64_t limit_arg() const { return limit_arg_; }

//   // Return the configured max burst, in resource units
//   uint64_t max_burst_arg() const { return max_burst_arg_; }

//   void set_resource(resource_t res) { resource_ = res; }

//   // Set the limit to `limit`, which is in units of the resource type
//   void set_limit(uint64_t limit) {
//     limit_arg_ = limit;
//     limit_ = to_work_units_per_cycle(limit);
//   }

//   // Set the max burst to `burst`, which is in units of the resource type
//   void set_max_burst(uint64_t burst) {
//     max_burst_arg_ = burst;
//     max_burst_ = to_work_units(burst);
//   }

//   TrafficClass *child() const { return child_; }

//   // Convert resource units to work units per cycle.
//   // Not meant to be used in the datapath: slow due to 128bit operations
//   static uint64_t to_work_units_per_cycle(uint64_t x) {
// #if INTPTR_MAX == INT64_MAX
//     return (static_cast<unsigned __int128>(x) << kUsageAmplifierPow) / tsc_hz;
// #elif INTPTR_MAX == INT32_MAX
//     // On 32bit systems, __int128 is not available.
//     // Instead, we sacrfice the accuracy of tsc_hz to avoid overflow
//     return (x << (kUsageAmplifierPow - 10)) / (tsc_hz >> 10);
// #else
// #error Forgot to add #include <cstdint>?
// #endif
//   }

//   // Convert resource units to work units
//   static uint64_t to_work_units(uint64_t x) { return x << kUsageAmplifierPow; }

//  private:
//   friend class Scheduler;

//   static const int kUsageAmplifierPow = 32;

//   // The resource that we are limiting.
//   resource_t resource_;

//   // 1 work unit = 2 ^ kUsageAmplifierPow resource usage.
//   // (for better precision without using floating point numbers)
//   uint64_t limit_;          // In work units per cycle (0 if unlimited).
//   uint64_t limit_arg_;      // In resource units per second.
//   uint64_t max_burst_;      // In work units.
//   uint64_t max_burst_arg_;  // In resource units.
//   uint64_t tokens_;         // In work units.

//   // Last time this TC was scheduled.
//   uint64_t last_tsc_;

//   TrafficClass *child_;
// };

// class LeafTrafficClass final : public TrafficClass {
//  public:
//   static const uint64_t kInitialWaitCycles = (1ull << 14);

//   explicit LeafTrafficClass(const std::string &name, Task *task)
//       : TrafficClass(name, POLICY_LEAF, false),
//         task_(task),
//         wait_cycles_(kInitialWaitCycles) {
//     task_->Attach(this);
//   }

//   ~LeafTrafficClass() override;

//   std::vector<TrafficClass *> Children() const override { return {}; }

//   // Returns true if child was removed successfully.
//   bool RemoveChild(TrafficClass *) override { return false; }

//   TrafficClass *PickNextChild() override { return nullptr; }

//   uint64_t wait_cycles() const { return wait_cycles_; }

//   void set_wait_cycles(uint64_t wait_cycles) { wait_cycles_ = wait_cycles; }

//   void BlockTowardsRoot() override {
//     TrafficClass::BlockTowardsRootSetBlocked(false);
//   }

//   void UnblockTowardsRoot(uint64_t tsc) override {
//     TrafficClass::UnblockTowardsRootSetBlocked(tsc, false);
//   }

//   Task *task() const { return task_; }

//   void FinishAndAccountTowardsRoot(SchedWakeupQueue *wakeup_queue,
//                                    [[maybe_unused]] TrafficClass *child,
//                                    resource_arr_t usage,
//                                    uint64_t tsc) override {
//     ACCUMULATE(stats_.usage, usage);
//     if (!parent_) {
//       return;
//     }
//     parent_->FinishAndAccountTowardsRoot(wakeup_queue, this, usage, tsc);
//   }

//  private:
//   Task *task_;

//   uint64_t wait_cycles_;
// };

// class PriorityChildArgs : public TCChildArgs {
//  public:
//   PriorityChildArgs(priority_t priority, TrafficClass *c)
//       : TCChildArgs(POLICY_PRIORITY, c), priority_(priority) {}
//   priority_t priority() { return priority_; }

//  private:
//   priority_t priority_;
// };

// class WeightedFairChildArgs : public TCChildArgs {
//  public:
//   WeightedFairChildArgs(resource_share_t share, TrafficClass *c)
//       : TCChildArgs(POLICY_WEIGHTED_FAIR, c), share_(share) {}
//   resource_share_t share() { return share_; }

//  private:
//   resource_share_t share_;
// };

// class RoundRobinChildArgs : public TCChildArgs {
//  public:
//   RoundRobinChildArgs(TrafficClass *c) : TCChildArgs(POLICY_ROUND_ROBIN, c) {}
// };

// class RateLimitChildArgs : public TCChildArgs {
//  public:
//   RateLimitChildArgs(TrafficClass *c) : TCChildArgs(POLICY_RATE_LIMIT, c) {}
// };

// // Responsible for creating and destroying all traffic classes.
// class TrafficClassBuilder {
//  public:
//   template <typename T, typename... TArgs>
//   static T *CreateTrafficClass(const std::string &name, TArgs... args) {
//     if (all_tcs_.count(name)) {
//       return nullptr;
//     }

//     T *c = new T(name, args...);
//     all_tcs_.emplace(name, c);
//     return c;
//   }

//   struct PriorityArgs {
//     PriorityFakeType dummy;
//   };
//   struct WeightedFairArgs {
//     WeightedFairFakeType dummy;
//     resource_t resource;
//   };
//   struct RoundRobinArgs {
//     RoundRobinFakeType dummy;
//   };
//   struct RateLimitArgs {
//     RateLimitFakeType dummy;
//     resource_t resource;
//     uint64_t limit;
//     uint64_t max_burst;
//   };

//   struct LeafArgs {
//     LeafFakeType dummy;
//     Task *task;
//   };

//   // These CreateTree(...) functions enable brace-initialized construction of a
//   // traffic class hierarchy.  For example,
//   //
//   //   CreateTree("foo", {PRIORITY}, {{PRIORITY, 10, ...}, {PRIORITY, 15, ...}})
//   //
//   // creates a tree with a priority root and two children, one of priority 10
//   // and one of priority 15, where the ... can contain a similar call to
//   // CreateTree to construct any similar subtree.  Classes that require
//   // arguments can be constructed as well; for example,
//   //
//   //   CreateTree("bar", {WEIGHTED_FAIR, RESOURCE_CYCLE}, {{WEIGHTED_FAIR, 3,
//   //   ...}})
//   //
//   // creates a tree with a weighted fair root sharing cycles and one child with
//   // a share of 3, with ... being additional calls to CreateTree.
//   //
//   // No checking is done on the tree to ensure any sort of validity.
//   //
//   // All classes constructed via these routines are created through calls to
//   // CreateTrafficClass above.
//   static TrafficClass *CreateTree(const std::string &name,
//                                   [[maybe_unused]] PriorityArgs args,
//                                   std::vector<PriorityChildArgs> children) {
//     PriorityTrafficClass *p = CreateTrafficClass<PriorityTrafficClass>(name);
//     for (auto &c : children) {
//       p->AddChild(c.child(), c.priority());
//     }
//     return p;
//   }

//   static TrafficClass *CreateTree(const std::string &name,
//                                   WeightedFairArgs args,
//                                   std::vector<WeightedFairChildArgs> children) {
//     WeightedFairTrafficClass *p =
//         CreateTrafficClass<WeightedFairTrafficClass>(name, args.resource);
//     for (auto &c : children) {
//       p->AddChild(c.child(), c.share());
//     }
//     return p;
//   }

//   static TrafficClass *CreateTree(const std::string &name,
//                                   [[maybe_unused]] RoundRobinArgs args,
//                                   std::vector<RoundRobinChildArgs> children =
//                                       std::vector<RoundRobinChildArgs>()) {
//     RoundRobinTrafficClass *p =
//         CreateTrafficClass<RoundRobinTrafficClass>(name);
//     for (auto &c : children) {
//       p->AddChild(c.child());
//     }
//     return p;
//   }

//   static TrafficClass *CreateTree(const std::string &name, RateLimitArgs args,
//                                   RateLimitChildArgs child) {
//     RateLimitTrafficClass *p = CreateTrafficClass<RateLimitTrafficClass>(
//         name, args.resource, args.limit, args.max_burst);
//     p->AddChild(child.child());
//     return p;
//   }

//   static TrafficClass *CreateTree(const std::string &name, LeafArgs args) {
//     return CreateTrafficClass<LeafTrafficClass>(name, args.task);
//   }

//   // Attempts to clear knowledge of all classes.  Returns true upon success.
//   // Frees all TrafficClass objects that were created by this builder.
//   static bool ClearAll();

//   // Attempts to clear knowledge of given class.  Returns true upon success.
//   static bool Clear(TrafficClass *c);

//   static const std::unordered_map<std::string, TrafficClass *> &all_tcs() {
//     return all_tcs_;
//   }

//   // Returns the TrafficClass * with the given name or nullptr if not found.
//   static TrafficClass *Find(const std::string &name) {
//     auto it = all_tcs_.find(name);
//     if (it != all_tcs_.end()) {
//       return it->second;
//     }
//     return nullptr;
//   }

//  private:
//   // A collection of all TCs in the system, mapped from their textual name.
//   static std::unordered_map<std::string, TrafficClass *> all_tcs_;
// };

// }  // namespace bess

// #endif  // BESS_TRAFFIC_CLASS_H_

// // ======================

// #include <algorithm>
// #include <cinttypes>
// #include <string>

// #include "opts.h"
// #include "scheduler.h"
// #include "utils/common.h"
// #include "utils/time.h"
// #include "worker.h"

// namespace bess {

// size_t TrafficClass::Size() const {
//   size_t ret = 1;  // itself
//   for (const auto *child : Children()) {
//     ret += child->Size();
//   };
//   return ret;
// }

// int TrafficClass::WorkerId() const {
//   for (int wid = 0; wid < Worker::kMaxWorkers; wid++) {
//     if (!is_worker_active(wid))
//       continue;

//     if (workers[wid]->scheduler()->root() == Root()) {
//       return wid;
//     }
//   }

//   // Orphan TC
//   return Worker::kAnyWorker;
// }

// PriorityTrafficClass::~PriorityTrafficClass() {
//   for (auto &c : children_) {
//     delete c.c_;
//   }
//   TrafficClassBuilder::Clear(this);
// }

// std::vector<TrafficClass *> PriorityTrafficClass::Children() const {
//   std::vector<TrafficClass *> ret;
//   for (const auto &child : children_) {
//     ret.push_back(child.c_);
//   }
//   return ret;
// }

// bool PriorityTrafficClass::AddChild(TrafficClass *child, priority_t priority) {
//   if (child->parent_) {
//     return false;
//   }

//   // Ensure that no child already has the given priority.
//   // FIXME: Allow having multiple TCs with the same priority.
//   //        (However, who gets scheduled first among them is not guaranteed)
//   for (const auto &c : children_) {
//     if (c.priority_ == priority) {
//       return false;
//     }
//   }

//   ChildData d{priority, child};
//   InsertSorted(children_, d);
//   child->parent_ = this;

//   UnblockTowardsRoot(rdtsc());

//   return true;
// }

// bool PriorityTrafficClass::RemoveChild(TrafficClass *child) {
//   if (child->parent_ != this) {
//     return false;
//   }

//   for (size_t i = 0; i < children_.size(); i++) {
//     if (children_[i].c_ == child) {
//       children_.erase(children_.begin() + i);
//       child->parent_ = nullptr;
//       if (first_runnable_ > i) {
//         first_runnable_--;
//       }
//       BlockTowardsRoot();

//       return true;
//     }
//   }

//   return false;
// }

// TrafficClass *PriorityTrafficClass::PickNextChild() {
//   return children_[first_runnable_].c_;
// }

// void PriorityTrafficClass::UnblockTowardsRoot(uint64_t tsc) {
//   size_t num_children = children_.size();
//   for (first_runnable_ = 0; first_runnable_ < num_children; ++first_runnable_) {
//     if (!children_[first_runnable_].c_->blocked_) {
//       break;
//     }
//   }
//   TrafficClass::UnblockTowardsRootSetBlocked(tsc,
//                                              first_runnable_ >= num_children);
// }

// void PriorityTrafficClass::BlockTowardsRoot() {
//   size_t num_children = children_.size();
//   while (first_runnable_ < num_children &&
//          children_[first_runnable_].c_->blocked_) {
//     ++first_runnable_;
//   }
//   TrafficClass::BlockTowardsRootSetBlocked(first_runnable_ == num_children);
// }

// void PriorityTrafficClass::FinishAndAccountTowardsRoot(
//     SchedWakeupQueue *wakeup_queue, TrafficClass *child, resource_arr_t usage,
//     uint64_t tsc) {
//   ACCUMULATE(stats_.usage, usage);

//   if (child->blocked_) {
//     // Find the next child that isn't blocked, if there is one.
//     size_t num_children = children_.size();
//     while (first_runnable_ < num_children &&
//            children_[first_runnable_].c_->blocked_) {
//       ++first_runnable_;
//     }
//     blocked_ = (first_runnable_ == num_children);
//   }
//   if (!parent_) {
//     return;
//   }
//   parent_->FinishAndAccountTowardsRoot(wakeup_queue, this, usage, tsc);
// }

// WeightedFairTrafficClass::~WeightedFairTrafficClass() {
//   while (!runnable_children_.empty()) {
//     delete runnable_children_.top().c;
//     runnable_children_.pop();
//   }
//   for (auto &c : blocked_children_) {
//     delete c.c;
//   }
//   TrafficClassBuilder::Clear(this);
// }

// std::vector<TrafficClass *> WeightedFairTrafficClass::Children() const {
//   std::vector<TrafficClass *> ret;
//   for (const auto &child : all_children_) {
//     ret.push_back(child.first);
//   }
//   return ret;
// }

// bool WeightedFairTrafficClass::AddChild(TrafficClass *child,
//                                         resource_share_t share) {
//   if (child->parent_ || share == 0) {
//     return false;
//   }

//   child->parent_ = this;
//   ChildData child_data{STRIDE1 / (double)share, {NextPass()}, child};
//   if (child->blocked_) {
//     blocked_children_.push_back(child_data);
//   } else {
//     runnable_children_.push(child_data);
//     UnblockTowardsRoot(rdtsc());
//   }

//   all_children_.emplace_back(child, share);

//   return true;
// }

// bool WeightedFairTrafficClass::RemoveChild(TrafficClass *child) {
//   if (child->parent_ != this) {
//     return false;
//   }

//   for (auto it = all_children_.begin(); it != all_children_.end(); it++) {
//     if (it->first == child) {
//       all_children_.erase(it);
//       break;
//     }
//   }

//   for (auto it = blocked_children_.begin(); it != blocked_children_.end();
//        it++) {
//     if (it->c == child) {
//       blocked_children_.erase(it);
//       child->parent_ = nullptr;
//       return true;
//     }
//   }

//   bool ret = runnable_children_.delete_single_element(
//       [=](const ChildData &x) { return x.c == child; });
//   if (ret) {
//     child->parent_ = nullptr;
//     BlockTowardsRoot();
//     return true;
//   }

//   return false;
// }

// TrafficClass *WeightedFairTrafficClass::PickNextChild() {
//   return runnable_children_.top().c;
// }

// void WeightedFairTrafficClass::UnblockTowardsRoot(uint64_t tsc) {
//   // TODO(barath): Optimize this unblocking behavior.
//   for (auto it = blocked_children_.begin(); it != blocked_children_.end();) {
//     if (!it->c->blocked_) {
//       it->pass = NextPass() + it->remain;
//       runnable_children_.push(*it);
//       blocked_children_.erase(it++);
//     } else {
//       ++it;
//     }
//   }

//   TrafficClass::UnblockTowardsRootSetBlocked(tsc, runnable_children_.empty());
// }

// void WeightedFairTrafficClass::BlockTowardsRoot() {
//   runnable_children_.delete_single_element([&](const ChildData &x) {
//     if (x.c->blocked_) {
//       blocked_children_.push_back(x);
//       return true;
//     }
//     return false;
//   });

//   TrafficClass::BlockTowardsRootSetBlocked(runnable_children_.empty());
// }

// void WeightedFairTrafficClass::FinishAndAccountTowardsRoot(
//     SchedWakeupQueue *wakeup_queue, TrafficClass *child, resource_arr_t usage,
//     uint64_t tsc) {
//   ACCUMULATE(stats_.usage, usage);

//   auto &item = runnable_children_.mutable_top();
//   uint64_t consumed = usage[resource_];
//   double pass_delta = item.stride * consumed / QUANTUM;

//   // DCHECK_EQ(item.c, child) << "Child that we picked should be at the front
//   // of priority queue.";
//   if (child->blocked_) {
//     // The blocked child will be penalized when unblocked, by the amount of the
//     // resource usage (pass_delta) not accounted for this round.
//     item.remain = pass_delta;
//     blocked_children_.emplace_back(std::move(item));
//     runnable_children_.pop();
//     blocked_ = runnable_children_.empty();
//   } else {
//     item.pass += pass_delta;
//     runnable_children_.decrease_key_top();
//   }

//   if (!parent_) {
//     return;
//   }
//   parent_->FinishAndAccountTowardsRoot(wakeup_queue, this, usage, tsc);
// }

// RoundRobinTrafficClass::~RoundRobinTrafficClass() {
//   for (TrafficClass *c : runnable_children_) {
//     delete c;
//   }
//   for (TrafficClass *c : blocked_children_) {
//     delete c;
//   }
//   TrafficClassBuilder::Clear(this);
// }

// bool RoundRobinTrafficClass::AddChild(TrafficClass *child) {
//   if (child->parent_) {
//     return false;
//   }
//   child->parent_ = this;

//   if (child->blocked_) {
//     blocked_children_.push_back(child);
//   } else {
//     runnable_children_.push_back(child);
//   }

//   UnblockTowardsRoot(rdtsc());

//   all_children_.push_back(child);

//   return true;
// }

// bool RoundRobinTrafficClass::RemoveChild(TrafficClass *child) {
//   if (child->parent_ != this) {
//     return false;
//   }

//   for (auto it = all_children_.begin(); it != all_children_.end(); it++) {
//     if (*it == child) {
//       all_children_.erase(it);
//       break;
//     }
//   }

//   for (auto it = blocked_children_.begin(); it != blocked_children_.end();
//        it++) {
//     if (*it == child) {
//       blocked_children_.erase(it);
//       child->parent_ = nullptr;
//       return true;
//     }
//   }

//   for (size_t i = 0; i < runnable_children_.size(); i++) {
//     if (runnable_children_[i] == child) {
//       runnable_children_.erase(runnable_children_.begin() + i);
//       child->parent_ = nullptr;
//       if (next_child_ > i) {
//         next_child_--;
//       }
//       // Wrap around for round robin.
//       if (next_child_ >= runnable_children_.size()) {
//         next_child_ = 0;
//       }
//       BlockTowardsRoot();

//       return true;
//     }
//   }

//   return false;
// }

// TrafficClass *RoundRobinTrafficClass::PickNextChild() {
//   return runnable_children_[next_child_];
// }

// void RoundRobinTrafficClass::UnblockTowardsRoot(uint64_t tsc) {
//   // TODO(barath): Optimize this unblocking behavior.
//   for (auto it = blocked_children_.begin(); it != blocked_children_.end();) {
//     if (!(*it)->blocked_) {
//       runnable_children_.push_back(*it);
//       it = blocked_children_.erase(it);
//     } else {
//       ++it;
//     }
//   }

//   TrafficClass::UnblockTowardsRootSetBlocked(tsc, runnable_children_.empty());
// }

// void RoundRobinTrafficClass::BlockTowardsRoot() {
//   for (size_t i = 0; i < runnable_children_.size();) {
//     if (runnable_children_[i]->blocked_) {
//       blocked_children_.push_back(runnable_children_[i]);
//       runnable_children_.erase(runnable_children_.begin() + i);
//       if (next_child_ > i) {
//         next_child_--;
//       }
//       // Wrap around for round robin.
//       if (next_child_ >= runnable_children_.size()) {
//         next_child_ = 0;
//       }
//     } else {
//       ++i;
//     }
//   }
//   TrafficClass::BlockTowardsRootSetBlocked(runnable_children_.empty());
// }

// void RoundRobinTrafficClass::FinishAndAccountTowardsRoot(
//     SchedWakeupQueue *wakeup_queue, TrafficClass *child, resource_arr_t usage,
//     uint64_t tsc) {
//   ACCUMULATE(stats_.usage, usage);
//   if (child->blocked_) {
//     runnable_children_.erase(runnable_children_.begin() + next_child_);
//     blocked_children_.push_back(child);
//     blocked_ = runnable_children_.empty();
//   } else {
//     next_child_ += usage[RESOURCE_COUNT];
//   }

//   // Wrap around for round robin.
//   if (next_child_ >= runnable_children_.size()) {
//     next_child_ = 0;
//   }

//   if (!parent_) {
//     return;
//   }
//   parent_->FinishAndAccountTowardsRoot(wakeup_queue, this, usage, tsc);
// }

// RateLimitTrafficClass::~RateLimitTrafficClass() {
//   // TODO(barath): Ensure that when this destructor is called this instance is
//   // also cleared out of the wakeup_queue_ in Scheduler if it is present
//   // there.
//   delete child_;
//   TrafficClassBuilder::Clear(this);
// }

// std::vector<TrafficClass *> RateLimitTrafficClass::Children() const {
//   if (child_ == nullptr) {
//     return {};
//   } else {
//     return {child_};
//   }
// }

// bool RateLimitTrafficClass::AddChild(TrafficClass *child) {
//   if (child->parent_ || child_ != nullptr) {
//     return false;
//   }

//   child_ = child;
//   child->parent_ = this;

//   UnblockTowardsRoot(rdtsc());

//   return true;
// }

// bool RateLimitTrafficClass::RemoveChild(TrafficClass *child) {
//   if (child->parent_ != this || child != child_) {
//     return false;
//   }

//   child_->parent_ = nullptr;
//   child_ = nullptr;

//   BlockTowardsRoot();

//   return true;
// }

// TrafficClass *RateLimitTrafficClass::PickNextChild() {
//   return child_;
// }

// void RateLimitTrafficClass::UnblockTowardsRoot(uint64_t tsc) {
//   last_tsc_ = tsc;

//   bool blocked = wakeup_time_ || !child_ || child_->blocked_;
//   TrafficClass::UnblockTowardsRootSetBlocked(tsc, blocked);
// }

// void RateLimitTrafficClass::BlockTowardsRoot() {
//   bool blocked = !child_ || child_->blocked_;
//   TrafficClass::BlockTowardsRootSetBlocked(blocked);
// }

// void RateLimitTrafficClass::FinishAndAccountTowardsRoot(
//     SchedWakeupQueue *wakeup_queue, TrafficClass *child, resource_arr_t usage,
//     uint64_t tsc) {
//   ACCUMULATE(stats_.usage, usage);
//   uint64_t elapsed_cycles = tsc - last_tsc_;
//   last_tsc_ = tsc;

//   uint64_t tokens = tokens_ + limit_ * elapsed_cycles;
//   uint64_t consumed = to_work_units(usage[resource_]);
//   if (tokens < consumed) {
//     // Exceeded limit, throttled.
//     tokens_ = 0;
//     blocked_ = true;
//     ++stats_.cnt_throttled;

//     if (limit_) {
//       uint64_t wait_tsc = (consumed - tokens) / limit_;
//       wakeup_time_ = tsc + wait_tsc;
//       wakeup_queue->Add(this);
//     }
//   } else {
//     // Still has some tokens, unthrottled.
//     tokens_ = std::min(tokens - consumed, max_burst_);
//   }

//   // Can still become blocked if the child was blocked, even if we haven't hit
//   // the rate limit.
//   blocked_ |= child->blocked_;

//   if (!parent_) {
//     return;
//   }
//   parent_->FinishAndAccountTowardsRoot(wakeup_queue, this, usage, tsc);
// }

// LeafTrafficClass::~LeafTrafficClass() {
//   TrafficClassBuilder::Clear(this);
//   task_->Detach();
//   delete task_;
// }

// std::unordered_map<std::string, TrafficClass *> TrafficClassBuilder::all_tcs_;

// bool TrafficClassBuilder::ClearAll() {
//   all_tcs_.clear();
//   return true;
// }

// bool TrafficClassBuilder::Clear(TrafficClass *c) {
//   bool ret = all_tcs_.erase(c->name());
//   return ret;
// }

// }  // namespace bess
