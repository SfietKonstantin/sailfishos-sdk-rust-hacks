FROM coderus/sailfishos-platform-sdk-aarch64

RUN sb2 -R zypper in --download-only -y rust cargo && \
    sb2 -R rm $(sb2 find  /var/cache/zypp/packages/jolla/oss/aarch64/ -name cpp-*.rpm) && \
    sb2 -R rm $(sb2 find  /var/cache/zypp/packages/jolla/oss/aarch64/ -name gcc-*.rpm) && \
    sb2 -R rpm -i --nodeps $(sb2 find  /var/cache/zypp/packages/jolla/oss/aarch64/ -name *.rpm) && \
    sb2 -R find  /var/cache/zypp/packages/jolla/oss/aarch64/ -name *.rpm -delete
