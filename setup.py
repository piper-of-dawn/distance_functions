# from setuptools import dist

# dist.Distribution().fetch_build_eggs(["setuptools_rust"])
# from setuptools import setup
# from setuptools_rust import Binding, RustExtension

# setup(
#     name="distance_functions",
#     version="0.1",
#     rust_extensions=[
#         RustExtension(
#             ".distance_functions.distance_functions",
#             path="Cargo.toml",
#             binding=Binding.PyO3,
#         )
#     ],
#     install_requires=["setuptools-rust>=0.10.1", "cffi>=1.0.0"],
#     packages=["distance_functions"],
#     classifiers=[
#         "License :: OSI Approved :: MIT License",
#         "Development Status :: 3 - Alpha",
#         "Intended Audience :: Developers",
#         "Programming Language :: Python",
#         "Programming Language :: Rust",
#         "Operating System :: POSIX",
#         "Operating System :: MacOS :: MacOS X",
#     ],
#     zip_safe=False,
# )
