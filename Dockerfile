# this docker file is just used to check that the compiled executable
# works on a system that does not have hdf5 lib installed.
# the original binary is compiled with a static hdf5 lib so this should be contained within the executable

# docker build -t hdf5_rust .
# docker run -it hdf5_rust

FROM ubuntu:latest

COPY target/debug/website_example .
COPY Li6.h5 .

CMD ./website_example