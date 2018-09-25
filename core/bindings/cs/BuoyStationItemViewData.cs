using System;
using System.Runtime.InteropServices;

internal class BuoyStationItemViewDataNativeInterface {
    [DllImport("buoyfinder")]
    internal extern static void buoy_station_item_view_data_free(IntPtr store);

    [DllImport("buoyfinder")]
    internal extern static String buoy_station_item_view_data_title(BuoyStationItemViewDataHandle handle);

    [DllImport("buoyfinder")]
    internal extern static String buoy_station_item_view_data_subtitle(BuoyStationItemViewDataHandle handle);

    [DllImport("buoyfinder")]
    internal extern static String buoy_station_item_view_data_on_click_id(BuoyStationItemViewDataHandle handle);

    [DllImport("buoyfinder")]
    internal extern static BuoyStationIcon buoy_station_item_view_data_icon(BuoyStationItemViewDataHandle handle);

    [DllImport("buoyfinder")]
    internal extern static Color buoy_station_item_view_data_color(BuoyStationItemViewDataHandle handle);

    [DllImport("buoyfinder")]
    internal extern static LocationHandle buoy_station_item_view_data_location(BuoyStationItemViewDataHandle handle);
}

public class BuoyStationItemViewDataHandle : SafeHandle {

    public BuoyStationItemViewDataHandle() : base(IntPtr.Zero, true) {}

    public override bool IsInvalid{
        get { 
            return false; 
        }
    }

    protected override bool ReleaseHandle() {
        BuoyStationItemViewDataNativeInterface.buoy_station_item_view_data_free(handle);
        return true;
    }
}

public class BuoyStationItemViewData : IDisposable {

    internal BuoyStationItemViewDataHandle hnd;

    public BuoyStationItemViewData(BuoyStationItemViewDataHandle handle) {
        hnd = handle;
    }

    void Dispose() {
        hnd.Dispose();
    }

    public String title {
        get {
            return BuoyStationItemViewDataNativeInterface.buoy_station_item_view_data_title(hnd);
        }
    }

    public String subtitle {
        get {
            return BuoyStationItemViewDataNativeInterface.buoy_station_item_view_data_subtitle(hnd);
        }
    }

    public String onClickId {
        get {
            return BuoyStationItemViewDataNativeInterface.buoy_station_item_view_data_on_click_id(hnd);
        }
    }

    public BuoyStationIcon icon {
        get {
            return BuoyStationItemViewDataNativeInterface.buoy_station_item_view_data_icon(hnd);
        }
    }

    public Color color {
        get {
            return BuoyStationItemViewDataNativeInterface.buoy_station_item_view_data_color(hnd);
        }
    }

    public String location {
        get {
            return Location(BuoyStationItemViewDataNativeInterface.buoy_station_item_view_data_location(hnd));
        }
    }
}