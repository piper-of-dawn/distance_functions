from setuptools import dist
dist.Distribution().fetch_build_eggs(['setuptools_rust'])
from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="distance_functions",
    version="0.1",
    rust_extensions=[RustExtension(
        ".distance_functions.distance_functions",
        path="Cargo.toml", binding=Binding.PyO3)],
    packages=["distance_functions"],
    classifiers=[
            "License :: OSI Approved :: MIT License",
            "Development Status :: 3 - Alpha",
            "Intended Audience :: Developers",
            "Programming Language :: Python",
            "Programming Language :: Rust",
            "Operating System :: POSIX",
            "Operating System :: MacOS :: MacOS X",
        ],
    # entry_points={
    #     'console_scripts': [
    #         'fib-number = flitton_fib_rs.fib_number_command:fib_number_command',
    #     ],
    # },
    zip_safe=False,
)