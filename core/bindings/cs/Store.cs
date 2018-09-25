using System;
using System.Runtime.InteropServices;

internal class StoreNativeInterface {
    [DllImport("buoyfinder")]
    internal extern static LocationHandle store_new();

    [DllImport("buoyfinder")]
    internal extern static void store_free(IntPtr store);

    [DllImport("buoyfinder")]
    internal extern static void fetch_buoy_stations(StoreHandle store);
}

internal class StoreHandle : SafeHandle {

    public StoreHandle() : base(IntPtr.Zero, true) {}

    public override bool IsInvalid {
        get { 
            return false; 
        }
    }

    protected override bool ReleaseHandle() {
        StoreNativeInterface.store_free(handle);
        return true;
    }
}

public class Store : IDisposable {

    private StoreHandle hnd;

    public Store() {
        hnd = StoreNativeInterface.store_new();
    }

    public void Dispose() {
       hnd.Dispose();
    }

    public void fetchBuoyStations() {
        StoreNativeInterface.fetch_buoy_stations(hnd);
    }

    public StoreHandle handle {
        get {
            return hnd;
        }
    }
}