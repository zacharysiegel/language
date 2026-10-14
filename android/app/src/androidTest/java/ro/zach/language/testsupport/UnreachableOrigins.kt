package ro.zach.language.testsupport

/** Origins no server answers, for tests that run without the Mac's servers. */
object UnreachableOrigins {
    /* Port 9 (discard) has no listener, so a connection to it is refused at once. */
    const val REFUSING_HOST: String = "127.0.0.1:9"
    const val REFUSING_ORIGIN: String = "http://$REFUSING_HOST"
    /* A TEST-NET-1 address (RFC 5737), never routed, so a connection to it waits rather than failing. */
    const val WAITING_ORIGIN: String = "http://192.0.2.1:3002"
}
