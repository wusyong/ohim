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

/// Error type of `DOMUrl`.
#[derive(Debug)]
pub enum UrlError {
    /// Error when setting user name.
    SetUsername,
    /// Error when setting IP Host.
    SetIpHost,
    /// Error when setting password.
    SetPassword,
    /// Error when convert to file path.
    ToFilePath,
    /// Error when convert from file path.
    FromFilePath,
}

/// A URL type used in DOM context.
#[derive(Clone, Eq, Hash, MallocSizeOf, Ord, PartialEq, PartialOrd)]
pub struct DOMUrl(#[conditional_malloc_size_of] Arc<Url>);

impl DOMUrl {
    /// Create a `DOMUrl` from provided `Url`.
    pub fn from_url(url: Url) -> Self {
        DOMUrl(Arc::new(url))
    }

    /// Create a `DOMUrl` from a string with a base.
    pub fn parse_with_base(base: Option<&Self>, input: &str) -> Result<Self, url::ParseError> {
        Url::options()
            .base_url(base.map(|b| &*b.0))
            .parse(input)
            .map(Self::from_url)
    }

    /// Convert into `String`.
    pub fn into_string(self) -> String {
        String::from(self.into_url())
    }

    /// Convert into `Url`.
    pub fn into_url(self) -> Url {
        self.as_url().clone()
    }

    /// Get the reference of `Url`.
    pub fn as_url(&self) -> &Url {
        &self.0
    }

    /// Create a `DOMUrl` from a string.
    pub fn parse(input: &str) -> Result<Self, url::ParseError> {
        Url::parse(input).map(Self::from_url)
    }

    /// Return whether this URL is a cannot-be-a-base URL,
    /// meaning that parsing a relative URL string with this URL as the base will return an error.
    ///
    /// This is the case if the scheme and `:` delimiter are not followed by a `/` slash,
    /// as is typically the case of `data:` and `mailto:` URLs.
    pub fn cannot_be_a_base(&self) -> bool {
        self.0.cannot_be_a_base()
    }

    /// If this URL has a host and it is a domain name (not an IP address), return it.
    /// Non-ASCII domains are punycode-encoded per IDNA if this is the host
    /// of a special URL, or percent encoded for non-special URLs.
    pub fn domain(&self) -> Option<&str> {
        self.0.domain()
    }

    /// Return this URL’s fragment identifier, if any.
    /// A fragment is the part of the URL after the `#` symbol.
    /// The fragment is optional and, if present, contains a fragment identifier
    /// that identifies a secondary resource, such as a section heading
    /// of a document.
    ///
    /// In HTML, the fragment identifier is usually the id attribute of a an element
    /// that is scrolled to on load. Browsers typically will not send the fragment portion
    /// of a URL to the server.
    ///
    /// **Note:** the parser did *not* percent-encode this component,
    /// but the input may have been percent-encoded already.
    pub fn fragment(&self) -> Option<&str> {
        self.0.fragment()
    }

    /// Return the path for this URL, as a percent-encoded ASCII string.
    /// For cannot-be-a-base URLs, this is an arbitrary string that doesn’t start with '/'.
    /// For other URLs, this starts with a '/' slash
    /// and continues with slash-separated path segments.
    pub fn path(&self) -> &str {
        self.0.path()
    }

    /// Get the origin of the URL.
    pub fn origin(&self) -> ImmutableOrigin {
        ImmutableOrigin::new(self.0.origin())
    }

    /// Return the scheme of this URL, lower-cased, as an ASCII string without the ':' delimiter.
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

    /// Return the serialization of this URL.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Get mutable reference of `Url`.
    pub fn as_mut_url(&mut self) -> &mut Url {
        Arc::make_mut(&mut self.0)
    }

    /// Change this URL’s username.
    ///
    /// If this URL is cannot-be-a-base or does not have a host, do nothing and return
    /// `UrlError:SetUsername`.
    pub fn set_username(&mut self, user: &str) -> Result<(), UrlError> {
        self.as_mut_url()
            .set_username(user)
            .map_err(|_| UrlError::SetUsername)
    }

    /// Change this URL’s host to the given IP address.
    ///
    /// If this URL is cannot-be-a-base, do nothing and return `UrlError::SetIpHost`.
    pub fn set_ip_host(&mut self, addr: IpAddr) -> Result<(), UrlError> {
        self.as_mut_url()
            .set_ip_host(addr)
            .map_err(|_| UrlError::SetIpHost)
    }

    /// Change this URL’s password.
    ///
    /// If this URL is cannot-be-a-base or does not have a host, do nothing and return
    /// `UrlError::SetPassword`.
    pub fn set_password(&mut self, pass: Option<&str>) -> Result<(), UrlError> {
        self.as_mut_url()
            .set_password(pass)
            .map_err(|_| UrlError::SetPassword)
    }

    /// Change this URL’s fragment identifier.
    pub fn set_fragment(&mut self, fragment: Option<&str>) {
        self.as_mut_url().set_fragment(fragment)
    }

    /// Return the username for this URL (typically the empty string)
    /// as a percent-encoded ASCII string.
    pub fn username(&self) -> &str {
        self.0.username()
    }

    /// Return the password for this URL, if any, as a percent-encoded ASCII string.
    pub fn password(&self) -> Option<&str> {
        self.0.password()
    }

    /// Assuming the URL is in the `file` scheme or similar,
    /// convert its path to an absolute `std::path::Path`.
    ///
    /// **Note:** This does not actually check the URL’s `scheme`,
    /// and may give nonsensical results for other schemes.
    /// It is the user’s responsibility to check the URL’s scheme before calling this.
    ///
    /// ```rust
    /// let path = url.to_file_path();
    /// ```
    ///
    /// Returns `Err` if the host is neither empty nor `"localhost"` (except on Windows, where
    /// `file:` URLs may have a non-local host),
    /// or if `Path::new_opt()` returns `None`.
    /// (That is, if the percent-decoded path contains a NUL byte or,
    /// for a Windows path, is not UTF-8.)
    pub fn to_file_path(&self) -> Result<::std::path::PathBuf, UrlError> {
        self.0.to_file_path().map_err(|_| UrlError::ToFilePath)
    }

    /// Return the parsed representation of the host for this URL.
    /// Non-ASCII domain labels are punycode-encoded per IDNA if this is the host
    /// of a special URL, or percent encoded for non-special URLs.
    ///
    /// Cannot-be-a-base URLs (typical of `data:` and `mailto:`) and some `file:` URLs
    /// don’t have a host.
    ///
    /// See also the `host_str` method.
    pub fn host(&self) -> Option<url::Host<&str>> {
        self.0.host()
    }

    /// Return the string representation of the host (domain or IP address) for this URL, if any.
    /// Non-ASCII domains are punycode-encoded per IDNA if this is the host
    /// of a special URL, or percent encoded for non-special URLs.
    /// IPv6 addresses are given between `[` and `]` brackets.
    ///
    /// Cannot-be-a-base URLs (typical of `data:` and `mailto:`) and some `file:` URLs
    /// don’t have a host.
    ///
    /// See also the `host` method.
    pub fn host_str(&self) -> Option<&str> {
        self.0.host_str()
    }

    /// Return the port number for this URL, if any.
    ///
    /// Note that default port numbers are never reflected by the serialization,
    /// use the `port_or_known_default()` method if you want a default port number returned.
    pub fn port(&self) -> Option<u16> {
        self.0.port()
    }

    /// Return the port number for this URL, or the default port number if it is known.
    ///
    /// This method only knows the default port number
    /// of the `http`, `https`, `ws`, `wss` and `ftp` schemes.
    ///
    /// For URLs in these schemes, this method always returns `Some(_)`.
    /// For other schemes, it is the same as `Url::port()`.
    pub fn port_or_known_default(&self) -> Option<u16> {
        self.0.port_or_known_default()
    }

    /// Parse a string as an URL, with this URL as the base URL.
    pub fn join(&self, input: &str) -> Result<DOMUrl, url::ParseError> {
        self.0.join(input).map(Self::from_url)
    }

    /// Unless this URL is cannot-be-a-base,
    /// return an iterator of '/' slash-separated path segments,
    /// each as a percent-encoded ASCII string.
    ///
    /// Return `None` for cannot-be-a-base URLs.
    ///
    /// When `Some` is returned, the iterator always contains at least one string
    /// (which may be empty).
    pub fn path_segments(&self) -> Option<std::str::Split<'_, char>> {
        self.0.path_segments()
    }

    /// Return this URL’s query string, if any, as a percent-encoded ASCII string.
    pub fn query(&self) -> Option<&str> {
        self.0.query()
    }

    /// Convert a file name as `std::path::Path` into an URL in the `file` scheme.
    ///
    /// This returns `Err` if the given path is not absolute or,
    /// on Windows, if the prefix is not a disk prefix (e.g. `C:`) or a UNC prefix (`\\`).
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

impl fmt::Display for DOMUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl fmt::Debug for DOMUrl {
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

impl Index<RangeFull> for DOMUrl {
    type Output = str;
    fn index(&self, _: RangeFull) -> &str {
        &self.0[..]
    }
}

impl Index<RangeFrom<Position>> for DOMUrl {
    type Output = str;
    fn index(&self, range: RangeFrom<Position>) -> &str {
        &self.0[range]
    }
}

impl Index<RangeTo<Position>> for DOMUrl {
    type Output = str;
    fn index(&self, range: RangeTo<Position>) -> &str {
        &self.0[range]
    }
}

impl Index<Range<Position>> for DOMUrl {
    type Output = str;
    fn index(&self, range: Range<Position>) -> &str {
        &self.0[range]
    }
}

impl From<Url> for DOMUrl {
    fn from(url: Url) -> Self {
        DOMUrl::from_url(url)
    }
}

impl From<Arc<Url>> for DOMUrl {
    fn from(url: Arc<Url>) -> Self {
        DOMUrl(url)
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
    /// Create a `ImmutableOrigin` from provided `Origin`.
    pub fn new(origin: Origin) -> ImmutableOrigin {
        match origin {
            Origin::Opaque(_) => ImmutableOrigin::new_opaque(),
            Origin::Tuple(scheme, host, port) => ImmutableOrigin::Tuple(scheme, host, port),
        }
    }

    /// Check if `other` has the same origin.
    pub fn same_origin(&self, other: &MutableOrigin) -> bool {
        self == other.immutable()
    }

    /// Check if `other` has the same origin domain.
    pub fn same_origin_domain(&self, other: &MutableOrigin) -> bool {
        !other.has_domain() && self == other.immutable()
    }

    /// Creates a new opaque origin that is only equal to itself.
    pub fn new_opaque() -> ImmutableOrigin {
        ImmutableOrigin::Opaque(OpaqueOrigin::Opaque(Uuid::new_v4()))
    }

    /// For use in mixed security context tests because data: URL workers inherit contexts
    pub fn new_opaque_data_url_worker() -> ImmutableOrigin {
        ImmutableOrigin::Opaque(OpaqueOrigin::SecureWorkerFromDataUrl(Uuid::new_v4()))
    }

    /// Get the scheme of the origin.
    pub fn scheme(&self) -> Option<&str> {
        match *self {
            ImmutableOrigin::Opaque(_) => None,
            ImmutableOrigin::Tuple(ref scheme, _, _) => Some(&**scheme),
        }
    }

    /// Get the host of the origin.
    pub fn host(&self) -> Option<&Host> {
        match *self {
            ImmutableOrigin::Opaque(_) => None,
            ImmutableOrigin::Tuple(_, ref host, _) => Some(host),
        }
    }

    /// Get the port of the origin.
    pub fn port(&self) -> Option<u16> {
        match *self {
            ImmutableOrigin::Opaque(_) => None,
            ImmutableOrigin::Tuple(_, _, port) => Some(port),
        }
    }

    /// Convert into `Origin`.
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
    /// An Opaque origin.
    Opaque(Uuid),
    /// Workers created from `data:` urls will have opaque origins but need to be treated
    /// as inheriting the secure context they were created in. This tracks that the origin
    /// was created in such a context
    SecureWorkerFromDataUrl(Uuid),
}
malloc_size_of_is_0!(OpaqueOrigin);

/// A representation of an [origin](https://html.spec.whatwg.org/multipage/#origin-2).
#[derive(Clone, Debug)]
pub struct MutableOrigin(Rc<(ImmutableOrigin, RefCell<Option<Host>>)>);

malloc_size_of_is_0!(MutableOrigin);

impl MutableOrigin {
    /// Create a `MutableOrigin` from `ImmutableOrigin`.
    pub fn new(origin: ImmutableOrigin) -> MutableOrigin {
        MutableOrigin(Rc::new((origin, RefCell::new(None))))
    }

    /// Get the reference of `ImmutableOrigin`.
    pub fn immutable(&self) -> &ImmutableOrigin {
        &(self.0).0
    }

    /// Return whether this origin is a (scheme, host, port) tuple
    /// (as opposed to an opaque origin).
    pub fn is_tuple(&self) -> bool {
        self.immutable().is_tuple()
    }

    /// Get the scheme of the origin.
    pub fn scheme(&self) -> Option<&str> {
        self.immutable().scheme()
    }

    /// Get the host of the origin.
    pub fn host(&self) -> Option<&Host> {
        self.immutable().host()
    }

    /// Get the port of the origin.
    pub fn port(&self) -> Option<u16> {
        self.immutable().port()
    }

    /// Check if `other` has the same origin.
    pub fn same_origin(&self, other: &MutableOrigin) -> bool {
        self.immutable() == other.immutable()
    }

    /// Check if `other` has the same origin domain.
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

    /// Get the domain of the origin.
    pub fn domain(&self) -> Option<Host> {
        (self.0).1.borrow().clone()
    }

    /// Set the domain of the origin.
    pub fn set_domain(&self, domain: Host) {
        *(self.0).1.borrow_mut() = Some(domain);
    }

    /// Check if the origin has domain.
    pub fn has_domain(&self) -> bool {
        (self.0).1.borrow().is_some()
    }

    /// Get the effective domain of the origin.
    pub fn effective_domain(&self) -> Option<Host> {
        self.immutable()
            .host()
            .map(|host| self.domain().unwrap_or_else(|| host.clone()))
    }
}
