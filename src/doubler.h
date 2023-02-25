const int FACTOR = 2;

enum OD_Error{
    No_Error = 0,
    Create_Error = 1,
    Mapping_Error = 2,
    Unlink_Error = 3,
};

extern "C" int doubler(int x);
extern "C" void access_shared_od();
extern "C" void create_shared_od();

