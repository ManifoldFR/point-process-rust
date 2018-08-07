import sys

from setuptools import setup
from setuptools.command.test import test as TestCommand

try:
    from setuptools_rust import RustExtension, Binding, Strip
except ImportError:
    import subprocess
    errno = subprocess.call([sys.executable, '-m', 'pip', 'install', 'setuptools-rust'])
    if errno:
        print("Please install the setuptools-rust package.")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension

setup_requires = [
    'setuptools',
    'setuptools-rust>=0.10.1'
]
install_requires = ['numpy']

setup(
    name='pointprocesses',
    version='0.1.0',
    author="ManifoldFR",
    rust_extensions=[
        RustExtension(
            'pointprocesses.timedependent',
            'Cargo.toml',
            strip=Strip.No,
            native=True),
        RustExtension(
            'pointprocesses.generalized',
            'Cargo.toml',
            strip=Strip.No,
            native=True),
        ],
    packages=['pointprocesses'],
    install_requires=install_requires,
    setup_requires=setup_requires,
    include_package_data=True,
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False
)