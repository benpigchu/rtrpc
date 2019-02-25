use crate::graph::*;
use bytes::*;

// A util trait used to get value from buffers
trait TryGet: Sized {
    fn try_get(buf: &mut Buf) -> Option<Self>;
}

// A util trait used to put value to buffers
trait Put {
    fn put(&self, buf: &mut BytesMut);
}

#[cfg(test)]
fn reconvert_test<T: TryGet + Put + PartialEq + std::fmt::Debug + Clone>(value: T) {
    assert_eq!(Some(value.clone()), decode(encode(value.clone())));
}

impl TryGet for f64 {
    fn try_get(buf: &mut Buf) -> Option<f64> {
        if buf.remaining() < 8 {
            None
        } else {
            Some(buf.get_f64_be())
        }
    }
}

impl Put for f64 {
    fn put(&self, buf: &mut BytesMut) {
        buf.reserve(8);
        buf.put_f64_be(*self);
    }
}

impl TryGet for u32 {
    fn try_get(buf: &mut Buf) -> Option<u32> {
        if buf.remaining() < 4 {
            None
        } else {
            Some(buf.get_u32_be())
        }
    }
}

impl TryGet for u8 {
    fn try_get(buf: &mut Buf) -> Option<u8> {
        if buf.remaining() < 1 {
            None
        } else {
            Some(buf.get_u8())
        }
    }
}

impl Put for str {
    fn put(&self, buf: &mut BytesMut) {
        buf.reserve(4 + self.len());
        buf.put_u32_be(self.len() as u32);
        buf.put_slice(self.as_bytes());
    }
}

impl Put for String {
    fn put(&self, buf: &mut BytesMut) {
        self.as_str().put(buf);
    }
}

impl TryGet for String {
    fn try_get(buf: &mut Buf) -> Option<String> {
        let length = u32::try_get(buf)? as usize;
        if buf.remaining() < length {
            None
        } else {
            let mut buffer = BytesMut::new();
            buffer.resize(length, 0x0);
            buf.copy_to_slice(&mut buffer[..]);
            Some(String::from_utf8_lossy(&buffer[..]).into_owned())
        }
    }
}

#[test]
fn string_convert() {
    let string = "Test";
    reconvert_test(String::from(string))
}

impl<T> Put for Option<T>
where
    T: Put,
{
    fn put(&self, buf: &mut BytesMut) {
        buf.reserve(1);
        match self {
            None => buf.put_u8(0),
            Some(data) => {
                buf.put_u8(1);
                data.put(buf)
            }
        }
    }
}

impl<T> TryGet for Option<T>
where
    T: TryGet,
{
    fn try_get(buf: &mut Buf) -> Option<Self> {
        let present = u8::try_get(buf)?;
        match present {
            0 => Some(None),
            _ => Some(Some(T::try_get(buf)?)),
        }
    }
}

#[test]
fn option_convert() {
    reconvert_test::<Option<String>>(None);
    reconvert_test(Some(String::from("test")))
}

impl<T, E> Put for Result<T, E>
where
    T: Put,
    E: Put,
{
    fn put(&self, buf: &mut BytesMut) {
        buf.reserve(1);
        match self {
            Err(err) => {
                buf.put_u8(0);
                err.put(buf)
            }
            Ok(data) => {
                buf.put_u8(1);
                data.put(buf)
            }
        }
    }
}

impl<T, E> TryGet for Result<T, E>
where
    T: TryGet,
    E: TryGet,
{
    fn try_get(buf: &mut Buf) -> Option<Self> {
        let ok = u8::try_get(buf)?;
        match ok {
            0 => Some(Err(E::try_get(buf)?)),
            _ => Some(Ok(T::try_get(buf)?)),
        }
    }
}

#[test]
fn result_convert() {
    reconvert_test::<Result<String, String>>(Ok(String::from("Ok")));
    reconvert_test::<Result<String, String>>(Err(String::from("Err")))
}

impl<T> Put for Vec<T>
where
    T: Put,
{
    fn put(&self, buf: &mut BytesMut) {
        buf.reserve(4);
        buf.put_u32_be(self.len() as u32);
        for item in self {
            item.put(buf)
        }
    }
}
impl<T> TryGet for Vec<T>
where
    T: TryGet,
{
    fn try_get(buf: &mut Buf) -> Option<Self> {
        let length = u32::try_get(buf)?;
        let mut vec = Vec::with_capacity(length as usize);
        for _ in 0..length {
            vec.push(T::try_get(buf)?)
        }
        Some(vec)
    }
}

#[test]
fn vec_convert() {
    reconvert_test::<Vec<String>>(vec![String::from("Test"), String::from("Test2")]);
    reconvert_test::<Vec<String>>(vec![]);
}

impl<T1, T2, T3> Put for (T1, T2, T3)
where
    T1: Put,
    T2: Put,
    T3: Put,
{
    fn put(&self, buf: &mut BytesMut) {
        let (first, second, third) = self;
        first.put(buf);
        second.put(buf);
        third.put(buf);
    }
}

impl<T1, T2, T3> TryGet for (T1, T2, T3)
where
    T1: TryGet,
    T2: TryGet,
    T3: TryGet,
{
    fn try_get(buf: &mut Buf) -> Option<Self> {
        let first = T1::try_get(buf)?;
        let second = T2::try_get(buf)?;
        let third = T3::try_get(buf)?;
        Some((first, second, third))
    }
}

#[test]
fn triple_convert() {
    reconvert_test((
        String::from("Test"),
        String::from("Test2"),
        String::from("Test3"),
    ));
}

impl Put for NegativeCycle {
    fn put(&self, _buf: &mut BytesMut) {}
}

impl TryGet for NegativeCycle {
    fn try_get(_buf: &mut Buf) -> Option<Self> {
        Some(NegativeCycle())
    }
}

impl Put for Graph {
    fn put(&self, buf: &mut BytesMut) {
        let vec: Vec<_> = self.clone().into();
        vec.put(buf);
    }
}

impl TryGet for Graph {
    fn try_get(buf: &mut Buf) -> Option<Self> {
        let vec: Vec<(String, String, f64)> = Vec::try_get(buf)?;
        Some(Graph::from(vec))
    }
}

fn encode<T: Put>(value: T) -> Bytes {
    let mut bytes = BytesMut::new();
    value.put(&mut bytes);
    bytes.freeze()
}

pub fn encode_request(graph: &Graph, start: &str, end: &str) -> Bytes {
    encode((graph.clone(), String::from(start), String::from(end)))
}

pub fn encode_respond(result: Result<Option<Vec<String>>, NegativeCycle>) -> Bytes {
    encode(result)
}

fn decode<T: TryGet>(bytes: Bytes) -> Option<T> {
    use std::io::Cursor;
    let mut cursor = Cursor::new(&*bytes);
    let value = T::try_get(&mut cursor);
    if !cursor.has_remaining() {
        value
    } else {
        None
    }
}

pub fn decode_request(bytes: Bytes) -> Option<(Graph, String, String)> {
    decode(bytes)
}

pub fn decode_respond(bytes: Bytes) -> Option<Result<Option<Vec<String>>, NegativeCycle>> {
    decode(bytes)
}
