cargo build --release

:: Julia DLL Copy
copy .\target\release\libbruges_rs.dll .\julia\lib\.

:: Python  DLL Copy
copy .\target\release\bruges_rspy.dll .\python\bruges_rs\bruges_rspy.pyd

