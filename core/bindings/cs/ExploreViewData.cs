using System;
using System.Runtime.InteropServices;

internal class ExploreViewDataNativeInterface {
    [DllImport("buoyfinder")]
    internal extern static void explore_view_data_free(IntPtr store);

    [DllImport("buoyfinder")]
    internal extern static Boolean explore_view_data_is_loading(ExploreViewDataHandle handle);

    [DllImport("buoyfinder")]
    internal extern static Int64 explore_view_data_station_count(ExploreViewDataHandle handle);

    [DllImport("buoyfinder")]
    internal extern static BuoyStationItemViewDataHandle explore_view_data_station_index(ExploreViewDataHandle handle, Int64 index);
}

public class ExploreViewDataHandle : SafeHandle {
    public ExploreViewDataHandle() : base(IntPtr.Zero, true) {}

    public override bool IsInvalid{
        get { 
            return false; 
        }
    }

    protected override bool ReleaseHandle() {
        ExploreViewDataNativeInterface.explore_view_data_free(handle);
        return true;
    }
}

public class ExploreViewData {

    internal ExploreViewDataHandle hnd;

    public ExploreViewData(ExploreViewDataHandle handle) {
        hnd = handle;
    }

    public void Dispose() {
        hnd.Dispose();
    }

    public Boolean isLoading {
        get {
            return ExploreViewDataNativeInterface.explore_view_data_is_loading(hnd);
        }
    }

    public Int64 stationCount {
        get {
            return ExploreViewDataNativeInterface.explore_view_data_station_count(hnd);
        }
    }

    public BuoyStationItemViewData stationAtIndex(Int64 index) {
        return BuoyStationItemViewData(ExploreViewDataNativeInterface.explore_view_data_station_index(index));
    }
}