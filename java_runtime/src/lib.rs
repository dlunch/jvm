#![no_std]
extern crate alloc;

pub mod classes;
mod runtime;

use java_runtime_base::JavaClassProto;

pub use runtime::Runtime;

pub(crate) type RuntimeContext = dyn Runtime;
pub(crate) type RuntimeClassProto = JavaClassProto<dyn Runtime>;

pub fn get_class_proto(name: &str) -> Option<RuntimeClassProto> {
    Some(match name {
        "java/io/ByteArrayInputStream" => self::classes::java::io::ByteArrayInputStream::as_proto(),
        "java/io/DataInputStream" => self::classes::java::io::DataInputStream::as_proto(),
        "java/io/EOFException" => self::classes::java::io::EOFException::as_proto(),
        "java/io/IOException" => self::classes::java::io::IOException::as_proto(),
        "java/io/InputStream" => self::classes::java::io::InputStream::as_proto(),
        "java/io/OutputStream" => self::classes::java::io::OutputStream::as_proto(),
        "java/io/PrintStream" => self::classes::java::io::PrintStream::as_proto(),
        "java/lang/Class" => self::classes::java::lang::Class::as_proto(),
        "java/lang/Exception" => self::classes::java::lang::Exception::as_proto(),
        "java/lang/IllegalArgumentException" => self::classes::java::lang::IllegalArgumentException::as_proto(),
        "java/lang/IndexOutOfBoundsException" => self::classes::java::lang::IndexOutOfBoundsException::as_proto(),
        "java/lang/Integer" => self::classes::java::lang::Integer::as_proto(),
        "java/lang/InterruptedException" => self::classes::java::lang::InterruptedException::as_proto(),
        "java/lang/Math" => self::classes::java::lang::Math::as_proto(),
        "java/lang/NullPointerException" => self::classes::java::lang::NullPointerException::as_proto(),
        "java/lang/Object" => self::classes::java::lang::Object::as_proto(),
        "java/lang/Runnable" => self::classes::java::lang::Runnable::as_proto(),
        "java/lang/Runtime" => self::classes::java::lang::Runtime::as_proto(),
        "java/lang/RuntimeException" => self::classes::java::lang::RuntimeException::as_proto(),
        "java/lang/SecurityException" => self::classes::java::lang::SecurityException::as_proto(),
        "java/lang/String" => self::classes::java::lang::String::as_proto(),
        "java/lang/StringBuffer" => self::classes::java::lang::StringBuffer::as_proto(),
        "java/lang/System" => self::classes::java::lang::System::as_proto(),
        "java/lang/Thread" => self::classes::java::lang::Thread::as_proto(),
        "java/lang/Throwable" => self::classes::java::lang::Throwable::as_proto(),
        "java/util/Calendar" => self::classes::java::util::Calendar::as_proto(),
        "java/util/Date" => self::classes::java::util::Date::as_proto(),
        "java/util/GregorianCalendar" => self::classes::java::util::GregorianCalendar::as_proto(),
        "java/util/Hashtable" => self::classes::java::util::Hashtable::as_proto(),
        "java/util/Random" => self::classes::java::util::Random::as_proto(),
        "java/util/Timer" => self::classes::java::util::Timer::as_proto(),
        "java/util/TimerTask" => self::classes::java::util::TimerTask::as_proto(),
        "java/util/Vector" => self::classes::java::util::Vector::as_proto(),
        _ => return None,
    })
}
