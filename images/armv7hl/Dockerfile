FROM coderus/sailfishos-platform-sdk-armv7hl

RUN sb2 -R zypper in --download-only -y rust cargo && \
    sb2 -R rm $(sb2 find  /var/cache/zypp/packages/jolla/oss/armv7hl/ -name cpp-*.rpm) && \
    sb2 -R rm $(sb2 find  /var/cache/zypp/packages/jolla/oss/armv7hl/ -name gcc-*.rpm) && \
    sb2 -R rpm -i --nodeps $(sb2 find  /var/cache/zypp/packages/jolla/oss/armv7hl/ -name *.rpm) && \
    sb2 -R find  /var/cache/zypp/packages/jolla/oss/armv7hl/ -name *.rpm -delete
