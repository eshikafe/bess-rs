

use bess_rs::*;

// #include <glog/logging.h>
// #include <google/protobuf/any.pb.h>
// #include <gtest/gtest_prod.h>

// use message;
// use module;
// use packet;
// use pb::port_msg_pb;
// use pktbatch;
// use utils::common;
// use utils::ether;

type queue_t = u8;

const MAX_QUEUES_PER_DIR: u8 = 128; // [0, 31] (for each RX/TX)

const DRIVER_FLAG_SELF_INC_STATS: u16 = 0x0001;
const DRIVER_FLAG_SELF_OUT_STATS: u16 = 0x0002;
const MAX_QUEUE_SIZE: u32 = 4096;

const ETH_ALEN: u8 = 6;

// The term RX/TX could be very confusing for a virtual switch.
// Instead, we use the "incoming/outgoing" convention:
// - incoming: outside -> BESS
// - outgoing: BESS -> outside
pub enum PacketDirection{
  Incoming,
  Outgoing,
  Dirs,
}

// struct Port;
// struct PortTest;

// using port_init_func_t =
//     pb_func_t<CommandResponse, Port, google::protobuf::Any>;

// template <typename T, typename P>
// static inline port_init_func_t PORT_INIT_FUNC(
//     CommandResponse (P::*fn)(const T &)) {
//   return [fn](Port *p, const google::protobuf::Any &arg) {
//     T arg_;
//     arg.UnpackTo(&arg_);
//     auto base_fn = std::mem_fn(fn);
//     return base_fn(static_cast<P *>(p), arg_);
//   };
// }

// A class to generate new Port objects of specific types.  Each instance can
// generate Port objects of a specific class and specification.  Represents a
// "driver" of that port.
// class PortBuilder {
//  public:
//   friend class PortTest;
//   friend class ZeroCopyVPortTest;
//   friend class PortBuilderTest;

//   PortBuilder(std::function<Port *()> port_generator,
//               const std::string &class_name, const std::string &name_template,
//               const std::string &help_text, port_init_func_t init_func)
//       : port_generator_(port_generator),
//         class_name_(class_name),
//         name_template_(name_template),
//         help_text_(help_text),
//         init_func_(init_func),
//         initialized_(false) {}

//   // Returns a new Port object of the type represented by this PortBuilder
//   // instance (of type class_name) with the Port instance's name set to the
//   // given name.
//   Port *CreatePort(const std::string &name) const;

//   // Adds the given Port to the global Port collection.  Takes ownership of the
//   // pointer.  Returns true upon success.
//   static bool AddPort(Port *p);

//   // Returns 0 upon success, -errno upon failure.
//   static int DestroyPort(Port *p);

//   // Generates a name for a new port given the driver name and its template.
//   static std::string GenerateDefaultPortName(
//       const std::string &driver_name, const std::string &default_template);

//   // Invokes one-time initialization of the corresponding port class.  Returns
//   // true upon success.
//   bool InitPortClass();

//   // Initializes all drivers.
//   static void InitDrivers();

//   // Should be called via ADD_DRIVER (once per driver file) to register the
//   // existence of this driver.  Always returns true;
//   static bool RegisterPortClass(std::function<Port *()> port_generator,
//                                 const std::string &class_name,
//                                 const std::string &name_template,
//                                 const std::string &help_text,
//                                 port_init_func_t init_func);

//   static const std::map<std::string, PortBuilder> &all_port_builders();

//   static const std::map<std::string, Port *> &all_ports();

//   const std::string &class_name() const { return class_name_; }
//   const std::string &name_template() const { return name_template_; }
//   const std::string &help_text() const { return help_text_; }
//   bool initialized() const { return initialized_; }

//   CommandResponse RunInit(Port *p, const google::protobuf::Any &arg) const {
//     return init_func_(p, arg);
//   }

//  private:
//   // To avoid the static initialization ordering problem, this pseudo-getter
//   // function contains the real static all_port_builders class variable and
//   // returns it, ensuring its construction before use.
//   //
//   // If reset is true, clears the store of all port builders; to be used for
//   // testing and for dynamic loading of "drivers".
//   static std::map<std::string, PortBuilder> &all_port_builders_holder(
//       bool reset = false);

//   // A function that emits a new Port object of the type class_name.
//   std::function<Port *()> port_generator_;

//   // Tracks all port instances.
//   static std::map<std::string, Port *> all_ports_;

//   std::string class_name_;     // The name of this Port class.
//   std::string name_template_;  // The port default name prefix.
//   std::string help_text_;      // Help text about this port type.

//   port_init_func_t init_func_;  // Initialization function of this Port class

//   bool initialized_;  // Has this port class been initialized via
//                       // InitPortClass()?
// };

// struct BatchHistogram
//     : public std::array<uint64_t, bess::PacketBatch::kMaxBurst + 1> {
//   BatchHistogram &operator+=(const BatchHistogram &rhs) {
//     for (size_t i = 0; i < size(); i++) {
//       (*this)[i] += rhs[i];
//     }
//     return *this;
//   }
// };



struct QueueStats {
  packets: u64,
  dropped: u64,  // Not all drivers support this for INC direction
  bytes: u64,    // It doesn't include Ethernet overhead
  // requested_hist: BatchHistogram,
  // actual_hist: BatchHistogram,
  // diff_hist: BatchHistogram,
}

struct LinkStatus {
  speed: u32,    // speed in mbps: 1000, 40000, etc. 0 for vports
  full_duplex: bool,  // full-duplex enabled?
  autoneg: bool,      // auto-negotiated speed and duplex?
  link_up: bool,      // link up?
}

struct Conf {
  mac_addr: macaddr::MacAddr, //bess::utils::Ethernet::Address,
  mtu: u32,
  admin_up: bool,
}

struct PortStats {
  inc: QueueStats,
  out: QueueStats,
}

// pub struct Port {
//   pub link_status: LinkStatus,
//   pub conf: Conf,
//   pub port_stats: PortStats,

  // overide this section to create a new driver -----------------------------
//   Port()
//       : port_stats_(),
//         conf_(),
//         name_(),
//         driver_arg_(),
//         port_builder_(),
//         num_queues(),
//         queue_size(),
//         users(),
//         queue_stats() {
//     conf_.mac_addr.Randomize();
//     conf_.mtu = kDefaultMtu;
//     conf_.admin_up = true;
//   }

//   virtual ~Port() {}

//   virtual void DeInit() = 0;

//   // For one-time initialization of the port's "driver" (optional).
//   virtual void InitDriver() {}

//   virtual void CollectStats(bool reset);

//   virtual int RecvPackets(queue_t qid, bess::Packet **pkts, int cnt) = 0;
//   virtual int SendPackets(queue_t qid, bess::Packet **pkts, int cnt) = 0;

//   // For custom incoming / outgoing queue sizes (optional).
//   virtual size_t DefaultIncQueueSize() const { return kDefaultIncQueueSize; }
//   virtual size_t DefaultOutQueueSize() const { return kDefaultOutQueueSize; }

//   virtual uint64_t GetFlags() const { return 0; }

//   /*!
//    * Get any placement constraints that need to be met when receiving from this
//    * port.
//    */
//   virtual placement_constraint GetNodePlacementConstraint() const {
//     return UNCONSTRAINED_SOCKET;
//   }

//   virtual LinkStatus GetLinkStatus() {
//     return LinkStatus{
//         .speed = 0,
//         .full_duplex = true,
//         .autoneg = true,
//         .link_up = true,
//     };
//   }

//   virtual CommandResponse UpdateConf(const Conf &) {
//     return CommandFailure(ENOTSUP);
//   }

//   CommandResponse InitWithGenericArg(const google::protobuf::Any &arg);

//   PortStats GetPortStats();

//   /* queues == nullptr if _all_ queues are being acquired/released */
//   int AcquireQueues(const struct module *m, packet_dir_t dir,
//                     const queue_t *queues, int num);

//   void ReleaseQueues(const struct module *m, packet_dir_t dir,
//                      const queue_t *queues, int num);

//   const std::string &name() const { return name_; }
//   const Conf &conf() const { return conf_; }
//   const google::protobuf::Any &driver_arg() const { return driver_arg_; }

//   uint64_t num_rx_queues() const { return num_queues[PACKET_DIR_INC]; }
//   uint64_t num_tx_queues() const { return num_queues[PACKET_DIR_OUT]; }

//   uint64_t rx_queue_size() const { return queue_size[PACKET_DIR_INC]; }
//   uint64_t tx_queue_size() const { return queue_size[PACKET_DIR_OUT]; }

//   const PortBuilder *port_builder() const { return port_builder_; }

//  protected:
//   friend class PortBuilder;

//   /* for stats that do NOT belong to any queues */
//   PortStats port_stats_;

//   // Current configuration
//   Conf conf_;

//  private:
//   static const size_t kDefaultIncQueueSize = 1024;
//   static const size_t kDefaultOutQueueSize = 1024;

//   static const uint32_t kDefaultMtu = 1500;

//   // Private methods, for use by PortBuilder.
//   void set_name(const std::string &name) { name_ = name; }
//   void set_port_builder(const PortBuilder *port_builder) {
//     port_builder_ = port_builder;
//   }

//   std::string name_;                  // The name of this port instance.
//   google::protobuf::Any driver_arg_;  // Driver specific configuration.

//   // Class-wide spec of this type of port.  Non-owning.
//   const PortBuilder *port_builder_;

//   DISALLOW_COPY_AND_ASSIGN(Port);

//   // FIXME: porting in progress ----------------------------
//  public:
//   queue_t num_queues[PACKET_DIRS];
//   size_t queue_size[PACKET_DIRS];

//   /* which modules are using this port?
//    * TODO: more robust gate keeping */
//   const struct module *users[PACKET_DIRS][MAX_QUEUES_PER_DIR];

//   struct QueueStats queue_stats[PACKET_DIRS][MAX_QUEUES_PER_DIR];
// };

// #define ADD_DRIVER(_DRIVER, _NAME_TEMPLATE, _HELP)                       \
//   bool __driver__##_DRIVER = PortBuilder::RegisterPortClass(             \
//       std::function<Port *()>([]() { return new _DRIVER(); }), #_DRIVER, \
//       _NAME_TEMPLATE, _HELP, PORT_INIT_FUNC(&_DRIVER::Init));

// #endif  // BESS_PORT_H_


// #include <glog/logging.h>

// #include <cassert>
// #include <cctype>
// #include <cerrno>
// #include <cstdio>
// #include <initializer_list>
// #include <memory>
// #include <sstream>
// #include <string>

// #include "message.h"

// std::map<std::string, Port *> PortBuilder::all_ports_;

// Port *PortBuilder::CreatePort(const std::string &name) const {
//   Port *p = port_generator_();
//   p->set_name(name);
//   p->set_port_builder(this);
//   return p;
// }

// bool PortBuilder::AddPort(Port *p) {
//   return all_ports_.insert({p->name(), p}).second;
// }

// int PortBuilder::DestroyPort(Port *p) {
//   for (packet_dir_t dir : {PACKET_DIR_INC, PACKET_DIR_OUT}) {
//     for (queue_t i = 0; i < p->num_queues[dir]; i++) {
//       if (p->users[dir][i]) {
//         return -EBUSY;
//       }
//     }
//   }

//   all_ports_.erase(p->name());
//   p->DeInit();
//   delete p;

//   return 0;
// }

// std::string PortBuilder::GenerateDefaultPortName(
//     const std::string &driver_name, const std::string &default_template) {
//   std::string name_template;

//   if (default_template == "") {
//     std::ostringstream ss;
//     char last_char = '\0';
//     for (auto t : driver_name) {
//       if (last_char != '\0' && islower(last_char) && isupper(t))
//         ss << '_';

//       ss << char(tolower(t));
//       last_char = t;
//     }
//     name_template = ss.str();
//   } else {
//     name_template = default_template;
//   }

//   for (int i = 0;; i++) {
//     std::ostringstream ss;
//     ss << name_template << i;
//     std::string name = ss.str();

//     if (!all_ports_.count(name)) {
//       return name;  // found an unallocated name!
//     }
//   }

//   promise_unreachable();
// }

// bool PortBuilder::InitPortClass() {
//   if (initialized_) {
//     return false;
//   }

//   std::unique_ptr<Port> p(port_generator_());
//   p->InitDriver();
//   initialized_ = true;
//   return true;
// }

// void PortBuilder::InitDrivers() {
//   for (auto &pair : all_port_builders()) {
//     if (!const_cast<PortBuilder &>(pair.second).InitPortClass()) {
//       LOG(WARNING) << "Initializing driver (port class) "
//                    << pair.second.class_name() << " failed.";
//     }
//   }
// }

// bool PortBuilder::RegisterPortClass(
//     std::function<Port *()> port_generator, const std::string &class_name,
//     const std::string &name_template, const std::string &help_text,
//     std::function<CommandResponse(Port *, const google::protobuf::Any &)>
//         init_func) {
//   all_port_builders_holder().emplace(
//       std::piecewise_construct, std::forward_as_tuple(class_name),
//       std::forward_as_tuple(port_generator, class_name, name_template,
//                             help_text, init_func));
//   return true;
// }

// const std::map<std::string, PortBuilder> &PortBuilder::all_port_builders() {
//   return all_port_builders_holder();
// }

// std::map<std::string, PortBuilder> &PortBuilder::all_port_builders_holder(
//     bool reset) {
//   // Maps from class names to port builders.  Tracks all port classes (via their
//   // PortBuilders).
//   static std::map<std::string, PortBuilder> all_port_builders;

//   if (reset) {
//     all_port_builders.clear();
//   }

//   return all_port_builders;
// }

// const std::map<std::string, Port *> &PortBuilder::all_ports() {
//   return all_ports_;
// }

// void Port::CollectStats(bool) {}

// CommandResponse Port::InitWithGenericArg(const google::protobuf::Any &arg) {
//   CommandResponse ret = port_builder_->RunInit(this, arg);
//   if (!ret.has_error()) {
//     driver_arg_ = arg;
//   }
//   return ret;
// }

// Port::PortStats Port::GetPortStats() {
//   CollectStats(false);

//   PortStats ret = port_stats_;

//   for (queue_t qid = 0; qid < num_queues[PACKET_DIR_INC]; qid++) {
//     const QueueStats &inc = queue_stats[PACKET_DIR_INC][qid];

//     ret.inc.packets += inc.packets;
//     ret.inc.dropped += inc.dropped;
//     ret.inc.bytes += inc.bytes;
//     ret.inc.requested_hist += inc.requested_hist;
//     ret.inc.actual_hist += inc.actual_hist;
//     ret.inc.diff_hist += inc.diff_hist;
//   }

//   for (queue_t qid = 0; qid < num_queues[PACKET_DIR_OUT]; qid++) {
//     const QueueStats &out = queue_stats[PACKET_DIR_OUT][qid];
//     ret.out.packets += out.packets;
//     ret.out.dropped += out.dropped;
//     ret.out.bytes += out.bytes;
//     ret.out.requested_hist += out.requested_hist;
//     ret.out.actual_hist += out.actual_hist;
//     ret.out.diff_hist += out.diff_hist;
//   }

//   return ret;
// }

// int Port::AcquireQueues(const struct module *m, packet_dir_t dir,
//                         const queue_t *queues, int num) {
//   queue_t qid;
//   int i;

//   if (dir != PACKET_DIR_INC && dir != PACKET_DIR_OUT) {
//     LOG(ERROR) << "Incorrect packet dir " << dir;
//     return -EINVAL;
//   }

//   if (queues == nullptr) {
//     for (qid = 0; qid < num_queues[dir]; qid++) {
//       const struct module *user;

//       user = users[dir][qid];

//       /* the queue is already being used by someone else? */
//       if (user && user != m) {
//         return -EBUSY;
//       }
//     }

//     for (qid = 0; qid < num_queues[dir]; qid++) {
//       users[dir][qid] = m;
//     }

//     return 0;
//   }

//   for (i = 0; i < num; i++) {
//     const struct module *user;

//     qid = queues[i];

//     if (qid >= num_queues[dir]) {
//       return -EINVAL;
//     }

//     user = users[dir][qid];

//     /* the queue is already being used by someone else? */
//     if (user && user != m) {
//       return -EBUSY;
//     }
//   }

//   for (i = 0; i < num; i++) {
//     qid = queues[i];
//     users[dir][qid] = m;
//   }

//   return 0;
// }

// void Port::ReleaseQueues(const struct module *m, packet_dir_t dir,
//                          const queue_t *queues, int num) {
//   queue_t qid;
//   int i;

//   if (dir != PACKET_DIR_INC && dir != PACKET_DIR_OUT) {
//     LOG(ERROR) << "Incorrect packet dir " << dir;
//     return;
//   }

//   if (queues == nullptr) {
//     for (qid = 0; qid < num_queues[dir]; qid++) {
//       if (users[dir][qid] == m)
//         users[dir][qid] = nullptr;
//     }

//     return;
//   }

//   for (i = 0; i < num; i++) {
//     qid = queues[i];
//     if (qid >= num_queues[dir])
//       continue;

//     if (users[dir][qid] == m)
//       users[dir][qid] = nullptr;
//   }
// }
