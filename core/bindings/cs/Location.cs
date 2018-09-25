using System;
using System.Runtime.InteropServices;

internal class LocationNativeInterface {
    [DllImport("buoyfinder")]
    internal extern static LocationHandle location_new(double lat, double lon, string name);

    [DllImport("buoyfinder")]
    internal extern static void location_free(IntPtr location);

    [DllImport("buoyfinder")]
    internal extern static double location_latitude(LocationHandle location);

    [DllImport("buoyfinder")]
    internal extern static double location_longitude(LocationHandle location);

    [DllImport("buoyfinder")]
    internal extern static double location_altitude(LocationHandle location);

    [DllImport("buoyfinder")]
    internal extern static string location_name(LocationHandle location);
}

public class LocationHandle : SafeHandle {

    public LocationHandle() : base(IntPtr.Zero, true) {}

    public override bool IsInvalid{
        get { 
            return false; 
        }
    }

    protected override bool ReleaseHandle() {
        LocationNativeInterface.location_free(handle);
        return true;
    }
}

public class Location : IDisposable {

    private LocationHandle hnd;

    public Location(LocationHandle handle) {
        hnd = handle;
    }

    public Location(Double lat, Double lon, String locationName) {
        hnd = LocationNativeInterface.location_new(lat, lon, locationName);
    }

    public void Dispose() {
        hnd.Dispose();
    }

    public Double latitude {
        get {
            LocationNativeInterface.location_latitude(hnd);
        }
    }

    public Double longitude {
        get {
            LocationNativeInterface.location_longitude(hnd);
        }
    }

    public Double altitude {
        get {
            LocationNativeInterface.location_altitude(hnd);
        }
    }

    public String name {
        get {
            LocationNativeInterface.location_name(hnd);
        }
    }
}