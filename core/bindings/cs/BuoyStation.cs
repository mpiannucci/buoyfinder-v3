using System;
using System.Runtime.InteropServices;

internal class BuoyStationNativeInterface {
    [DllImport("buoyfinder")]
    internal extern static void buoy_station_new(String stationId, String stationName, Double latitude, Double longitude);

    [DllImport("buoyfinder")]
    internal extern static void buoy_station_free(IntPtr store);

    [DllImport("buoyfinder")]
    internal extern static String buoy_station_station_id(BuoyStationHandle handle);

    [DllImport("buoyfinder")]
    internal extern static String buoy_station_station_name(BuoyStationHandle handle);

    [DllImport("buoyfinder")]
    internal extern static Boolean buoy_station_is_active(BuoyStationHandle handle);    
}

public class BuoyStationHandle : SafeHandle {

    public BuoyStationHandle() : base(IntPtr.Zero, true) {}

    public override bool IsInvalid{
        get { 
            return false; 
        }
    }

    protected override bool ReleaseHandle() {
        BuoyStationNativeInterface.buoy_station_free(handle);
        return true;
    }
}

public class BuoyStation : IDisposable {

    internal BuoyStationHandle hnd;

    public BuoyStation(BuoyStationHandle handle) {
        hnd = handle;
    }

    public BuoyStation(String stationId, String stationName, Double lat, Double lon) {
        hnd = BuoyStationNativeInterface.buoy_station_new(stationId, stationName, lat, lon);
    }

    public void Dispose() {
        hnd.Dispose();
    }

    public String stationId {
        get {
            return BuoyStationNativeInterface.buoy_station_station_id(hnd);
        }
    }

    public String name {
        get {
            return BuoyStationNativeInterface.buoy_station_station_name(hnd);
        }
    }

    public Boolean isActive {
        get {
            return BuoyStationNativeInterface.buoy_station_is_active(hnd);
        }
    }
}