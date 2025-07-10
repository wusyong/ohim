//! URL related types
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::Hasher;
use std::net::IpAddr;
use std::ops::{Index, Range, RangeFrom, RangeFull, RangeTo};
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use malloc_size_of::malloc_size_of_is_0;
use malloc_size_of_derive::MallocSizeOf;
pub use url::Host;
use url::{Origin, Position, Url};
use uuid::Uuid;

const DATA_URL_DISPLAY_LENGTH: usize = 40;

// TODO: documentation
#[derive(Debug)]
pub enum UrlError {
    SetUsername,
    SetIpHost,
    SetPassword,
    ToFilePath,
    FromFilePath,
}

#[derive(Clone, Eq, Hash, MallocSizeOf, Ord, PartialEq, PartialOrd)]
pub struct DomUrl(#[conditional_malloc_size_of] Arc<Url>);

impl DomUrl {
    pub fn from_url(url: Url) -> Self {
        DomUrl(Arc::new(url))
    }

    pub fn parse_with_base(base: Option<&Self>, input: &str) -> Result<Self, url::ParseError> {
        Url::options()
            .base_url(base.map(|b| &*b.0))
            .parse(input)
            .map(Self::from_url)
    }

    pub fn into_string(self) -> String {
        String::from(self.into_url())
    }

    pub fn into_url(self) -> Url {
        self.as_url().clone()
    }

    pub fn get_arc(&self) -> Arc<Url> {
        self.0.clone()
    }

    pub fn as_url(&self) -> &Url {
        &self.0
    }

    pub fn parse(input: &str) -> Result<Self, url::ParseError> {
        Url::parse(input).map(Self::from_url)
    }

    pub fn cannot_be_a_base(&self) -> bool {
        self.0.cannot_be_a_base()
    }

    pub fn domain(&self) -> Option<&str> {
        self.0.domain()
    }

    pub fn fragment(&self) -> Option<&str> {
        self.0.fragment()
    }

    pub fn path(&self) -> &str {
        self.0.path()
    }

    pub fn origin(&self) -> ImmutableOrigin {
        ImmutableOrigin::new(self.0.origin())
    }

    pub fn scheme(&self) -> &str {
        self.0.scheme()
    }

    /// Check if scheme is "https" or "wss".
    pub fn is_secure_scheme(&self) -> bool {
        let scheme = self.scheme();
        scheme == "https" || scheme == "wss"
    }

    /// <https://fetch.spec.whatwg.org/#local-scheme>
    pub fn is_local_scheme(&self) -> bool {
        let scheme = self.scheme();
        scheme == "about" || scheme == "blob" || scheme == "data"
    }

    /// <https://url.spec.whatwg.org/#special-scheme>
    pub fn is_special_scheme(&self) -> bool {
        let scheme = self.scheme();
        scheme == "ftp"
            || scheme == "file"
            || scheme == "http"
            || scheme == "https"
            || scheme == "ws"
            || scheme == "wss"
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn as_mut_url(&mut self) -> &mut Url {
        Arc::make_mut(&mut self.0)
    }

    pub fn set_username(&mut self, user: &str) -> Result<(), UrlError> {
        self.as_mut_url()
            .set_username(user)
            .map_err(|_| UrlError::SetUsername)
    }

    pub fn set_ip_host(&mut self, addr: IpAddr) -> Result<(), UrlError> {
        self.as_mut_url()
            .set_ip_host(addr)
            .map_err(|_| UrlError::SetIpHost)
    }

    pub fn set_password(&mut self, pass: Option<&str>) -> Result<(), UrlError> {
        self.as_mut_url()
            .set_password(pass)
            .map_err(|_| UrlError::SetPassword)
    }

    pub fn set_fragment(&mut self, fragment: Option<&str>) {
        self.as_mut_url().set_fragment(fragment)
    }

    pub fn username(&self) -> &str {
        self.0.username()
    }

    pub fn password(&self) -> Option<&str> {
        self.0.password()
    }

    pub fn to_file_path(&self) -> Result<::std::path::PathBuf, UrlError> {
        self.0.to_file_path().map_err(|_| UrlError::ToFilePath)
    }

    pub fn host(&self) -> Option<url::Host<&str>> {
        self.0.host()
    }

    pub fn host_str(&self) -> Option<&str> {
        self.0.host_str()
    }

    pub fn port(&self) -> Option<u16> {
        self.0.port()
    }

    pub fn port_or_known_default(&self) -> Option<u16> {
        self.0.port_or_known_default()
    }

    pub fn join(&self, input: &str) -> Result<DomUrl, url::ParseError> {
        self.0.join(input).map(Self::from_url)
    }

    pub fn path_segments(&self) -> Option<std::str::Split<'_, char>> {
        self.0.path_segments()
    }

    pub fn query(&self) -> Option<&str> {
        self.0.query()
    }

    pub fn from_file_path<P: AsRef<Path>>(path: P) -> Result<Self, UrlError> {
        Url::from_file_path(path)
            .map(Self::from_url)
            .map_err(|_| UrlError::FromFilePath)
    }

    /// Return a non-standard shortened form of the URL. Mainly intended to be
    /// used for debug printing in a constrained space (e.g., thread names).
    pub fn debug_compact(&self) -> impl std::fmt::Display + '_ {
        match self.scheme() {
            "http" | "https" => {
                // Strip `scheme://`, which is hardly useful for identifying websites
                let mut st = self.as_str();
                st = st.strip_prefix(self.scheme()).unwrap_or(st);
                st = st.strip_prefix(':').unwrap_or(st);
                st = st.trim_start_matches('/');

                // Don't want to return an empty string
                if st.is_empty() {
                    st = self.as_str();
                }

                st
            }
            "file" => {
                // The only useful part in a `file` URL is usually only the last
                // few components
                let path = self.path();
                let i = path.rfind('/');
                let i = i.map(|i| path[..i].rfind('/').unwrap_or(i));
                match i {
                    None | Some(0) => path,
                    Some(i) => &path[i + 1..],
                }
            }
            _ => self.as_str(),
        }
    }

    /// <https://w3c.github.io/webappsec-secure-contexts/#potentially-trustworthy-url>
    pub fn is_potentially_trustworthy(&self) -> bool {
        // Step 1
        if self.as_str() == "about:blank" || self.as_str() == "about:srcdoc" {
            return true;
        }
        // Step 2
        if self.scheme() == "data" {
            return true;
        }
        // Step 3
        self.origin().is_potentially_trustworthy()
    }
}

impl fmt::Display for DomUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl fmt::Debug for DomUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let url_string = self.0.as_str();
        if self.scheme() != "data" || url_string.len() <= DATA_URL_DISPLAY_LENGTH {
            return url_string.fmt(formatter);
        }

        let mut hasher = DefaultHasher::new();
        hasher.write(self.0.as_str().as_bytes());

        format!(
            "{}... ({:x})",
            url_string
                .chars()
                .take(DATA_URL_DISPLAY_LENGTH)
                .collect::<String>(),
            hasher.finish()
        )
        .fmt(formatter)
    }
}

impl Index<RangeFull> for DomUrl {
    type Output = str;
    fn index(&self, _: RangeFull) -> &str {
        &self.0[..]
    }
}

impl Index<RangeFrom<Position>> for DomUrl {
    type Output = str;
    fn index(&self, range: RangeFrom<Position>) -> &str {
        &self.0[range]
    }
}

impl Index<RangeTo<Position>> for DomUrl {
    type Output = str;
    fn index(&self, range: RangeTo<Position>) -> &str {
        &self.0[range]
    }
}

impl Index<Range<Position>> for DomUrl {
    type Output = str;
    fn index(&self, range: Range<Position>) -> &str {
        &self.0[range]
    }
}

impl From<Url> for DomUrl {
    fn from(url: Url) -> Self {
        DomUrl::from_url(url)
    }
}

impl From<Arc<Url>> for DomUrl {
    fn from(url: Arc<Url>) -> Self {
        DomUrl(url)
    }
}

/// The origin of an URL
#[derive(Clone, Debug, Eq, Hash, MallocSizeOf, PartialEq)]
pub enum ImmutableOrigin {
    /// A globally unique identifier
    Opaque(OpaqueOrigin),

    /// Consists of the URL's scheme, host and port
    Tuple(String, Host, u16),
}

impl ImmutableOrigin {
    pub fn new(origin: Origin) -> ImmutableOrigin {
        match origin {
            Origin::Opaque(_) => ImmutableOrigin::new_opaque(),
            Origin::Tuple(scheme, host, port) => ImmutableOrigin::Tuple(scheme, host, port),
        }
    }

    pub fn same_origin(&self, other: &MutableOrigin) -> bool {
        self == other.immutable()
    }

    pub fn same_origin_domain(&self, other: &MutableOrigin) -> bool {
        !other.has_domain() && self == other.immutable()
    }

    /// Creates a new opaque origin that is only equal to itself.
    pub fn new_opaque() -> ImmutableOrigin {
        ImmutableOrigin::Opaque(OpaqueOrigin::Opaque(Uuid::new_v4()))
    }

    // For use in mixed security context tests because data: URL workers inherit contexts
    pub fn new_opaque_data_url_worker() -> ImmutableOrigin {
        ImmutableOrigin::Opaque(OpaqueOrigin::SecureWorkerFromDataUrl(Uuid::new_v4()))
    }

    pub fn scheme(&self) -> Option<&str> {
        match *self {
            ImmutableOrigin::Opaque(_) => None,
            ImmutableOrigin::Tuple(ref scheme, _, _) => Some(&**scheme),
        }
    }

    pub fn host(&self) -> Option<&Host> {
        match *self {
            ImmutableOrigin::Opaque(_) => None,
            ImmutableOrigin::Tuple(_, ref host, _) => Some(host),
        }
    }

    pub fn port(&self) -> Option<u16> {
        match *self {
            ImmutableOrigin::Opaque(_) => None,
            ImmutableOrigin::Tuple(_, _, port) => Some(port),
        }
    }

    pub fn into_url_origin(self) -> Origin {
        match self {
            ImmutableOrigin::Opaque(_) => Origin::new_opaque(),
            ImmutableOrigin::Tuple(scheme, host, port) => Origin::Tuple(scheme, host, port),
        }
    }

    /// Return whether this origin is a (scheme, host, port) tuple
    /// (as opposed to an opaque origin).
    pub fn is_tuple(&self) -> bool {
        match *self {
            ImmutableOrigin::Opaque(..) => false,
            ImmutableOrigin::Tuple(..) => true,
        }
    }

    /// <https://w3c.github.io/webappsec-secure-contexts/#is-origin-trustworthy>
    pub fn is_potentially_trustworthy(&self) -> bool {
        // 1. If origin is an opaque origin return "Not Trustworthy"
        if matches!(self, ImmutableOrigin::Opaque(_)) {
            return false;
        }

        if let ImmutableOrigin::Tuple(scheme, host, _) = self {
            // 3. If origin’s scheme is either "https" or "wss", return "Potentially Trustworthy"
            if scheme == "https" || scheme == "wss" {
                return true;
            }
            // 6. If origin’s scheme is "file", return "Potentially Trustworthy".
            if scheme == "file" {
                return true;
            }

            // 4. If origin’s host matches one of the CIDR notations 127.0.0.0/8 or ::1/128,
            // return "Potentially Trustworthy".
            if let Ok(ip_addr) = host.to_string().parse::<IpAddr>() {
                return ip_addr.is_loopback();
            }
            // 5. If the user agent conforms to the name resolution rules in
            // [let-localhost-be-localhost] and one of the following is true:
            // * origin’s host is "localhost" or "localhost."
            // * origin’s host ends with ".localhost" or ".localhost."
            // then return "Potentially Trustworthy".
            if let Host::Domain(domain) = host {
                if domain == "localhost" || domain.ends_with(".localhost") {
                    return true;
                }
            }
        }
        // 9. Return "Not Trustworthy".
        false
    }

    /// <https://html.spec.whatwg.org/multipage/#ascii-serialisation-of-an-origin>
    pub fn ascii_serialization(&self) -> String {
        self.clone().into_url_origin().ascii_serialization()
    }
}

/// Opaque identifier for URLs that have file or other schemes
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum OpaqueOrigin {
    Opaque(Uuid),
    // Workers created from `data:` urls will have opaque origins but need to be treated
    // as inheriting the secure context they were created in. This tracks that the origin
    // was created in such a context
    SecureWorkerFromDataUrl(Uuid),
}
malloc_size_of_is_0!(OpaqueOrigin);

/// A representation of an [origin](https://html.spec.whatwg.org/multipage/#origin-2).
#[derive(Clone, Debug)]
pub struct MutableOrigin(Rc<(ImmutableOrigin, RefCell<Option<Host>>)>);

malloc_size_of_is_0!(MutableOrigin);

impl MutableOrigin {
    pub fn new(origin: ImmutableOrigin) -> MutableOrigin {
        MutableOrigin(Rc::new((origin, RefCell::new(None))))
    }

    pub fn immutable(&self) -> &ImmutableOrigin {
        &(self.0).0
    }

    pub fn is_tuple(&self) -> bool {
        self.immutable().is_tuple()
    }

    pub fn scheme(&self) -> Option<&str> {
        self.immutable().scheme()
    }

    pub fn host(&self) -> Option<&Host> {
        self.immutable().host()
    }

    pub fn port(&self) -> Option<u16> {
        self.immutable().port()
    }

    pub fn same_origin(&self, other: &MutableOrigin) -> bool {
        self.immutable() == other.immutable()
    }

    pub fn same_origin_domain(&self, other: &MutableOrigin) -> bool {
        if let Some(ref self_domain) = *(self.0).1.borrow() {
            if let Some(ref other_domain) = *(other.0).1.borrow() {
                self_domain == other_domain
                    && self.immutable().scheme() == other.immutable().scheme()
            } else {
                false
            }
        } else {
            self.immutable().same_origin_domain(other)
        }
    }

    pub fn domain(&self) -> Option<Host> {
        (self.0).1.borrow().clone()
    }

    pub fn set_domain(&self, domain: Host) {
        *(self.0).1.borrow_mut() = Some(domain);
    }

    pub fn has_domain(&self) -> bool {
        (self.0).1.borrow().is_some()
    }

    pub fn effective_domain(&self) -> Option<Host> {
        self.immutable()
            .host()
            .map(|host| self.domain().unwrap_or_else(|| host.clone()))
    }
}
