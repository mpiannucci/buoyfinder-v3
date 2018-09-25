using System;
using System.Runtime.InteropServices;

internal class ExploreViewNativeInterface {
    [DllImport("buoyfinder")]
    internal extern static ExploreViewHandle explore_view_bind(ExploreViewWrapper exploreView, StoreHandle store);

    [DllImport("buoyfinder")]
    internal extern static void explore_view_unbind(ExploreViewHandle handle, StoreHandle store);
}

delegate void NewExploreViewDataCallback(ExploreViewDataHandle rawViewData);

[StructLayout(LayoutKind.Sequential)]
public struct ExploreViewWrapper{
    IntPtr view;
    NewExploreViewDataCallback newExploreViewDataCallback;
}

public class ExploreViewHandle : SafeHandle {

    public StoreHandle storeHandle;
    
    public ExploreViewHandle() : base(IntPtr.Zero, true) {
        storeHandle = IntPtr.Zero;
    }

    public override bool IsInvalid {
        get { 
            return false; 
        }
    }

    protected override bool ReleaseHandle() {
        ExploreViewNativeInterface.explore_view_unbind(handle, storeHandle);
        return true;
    }
}

// TODO: WRITE EXPLORE VIEW HANDLE MANAGER CLASS
public class ExploreViewManager : IDisposable {

    internal ExploreViewHandle viewHandle;
    internal IExploreView view;
    internal Store store;
    internal NewExploreViewDataCallback exploreViewDataCallback;

    ExploreViewManager(Store _store, IExploreView exploreView) {
        store = _store;
        view = exploreView;
        exploreViewDataCallback = new NewExploreViewDataCallback(handleNewViewData);
        ExploreViewWrapper wrapper = new ExploreViewWrapper(null, exploreViewDataCallback);
        viewHandle = ExploreViewNativeInterface.explore_view_bind(wrapper, store.handle);
        viewHandle.storeHandle = store.handle;
    }

    public void Dispose() {
        viewHandle.Dispose();
    }

    public void handleNewViewData(ExploreViewDataHandle rawViewData) {
        ExploreViewData newViewData = ExploreViewData(rawViewData);
        view.newViewData(newViewData);
    }
}

public interface IExploreView {
    public void newViewData(ExploreViewData viewData);
}
