import sys

from setuptools import setup

try:
    from setuptools_rust import RustExtension, Binding, Strip
except ImportError:
    import subprocess
    errno = subprocess.call([sys.executable, '-m', 'pip', 'install', 'setuptools-rust'])
    if errno:
        print("Please install the setuptools-rust package.")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension, Binding, Strip

try:
    import toml
except ImportError:
    import subprocess
    errno = subprocess.call([sys.executable, '-m', 'pip', 'install', 'toml'])
    if errno:
        print("Please install the toml package.")
        raise SystemExit(errno)
    else:
        import toml

cfg = toml.load("Cargo.toml")
package_meta = cfg['package']

setup_requires = [
    'setuptools',
    'setuptools-rust>=0.10.1'
]
install_requires = ['numpy']

setup(
    name=package_meta['name'],
    version=package_meta['version'],
    author=package_meta['authors'][0],
    rust_extensions=[
        RustExtension(
            'pointprocesses.timedependent',
            'Cargo.toml',
            binding=Binding.PyO3,
            strip=Strip.Debug),
        RustExtension(
            'pointprocesses.spatial',
            'Cargo.toml',
            binding=Binding.PyO3,
            strip=Strip.Debug),
        RustExtension(
            'pointprocesses.likelihood',
            'Cargo.toml',
            binding=Binding.PyO3,
            strip=Strip.Debug),
        ],
    packages=['pointprocesses'],
    install_requires=install_requires,
    setup_requires=setup_requires,
    include_package_data=True,
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False
)
