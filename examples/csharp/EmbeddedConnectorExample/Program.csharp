using Buttplug.Client;

namespace EmbeddedConnectorExample
{
    internal class Program
    {
        private static void Main()
        {
            // First off, we'll set up our Embedded Connector.
            var connector = new ButtplugEmbeddedConnector("Example Server");

            // The C# Buttplug library will automatically add all available
            // (nuget package referenced) DeviceSubtypeManagers for you. For
            // example, if you add a reference to the UWPBluetoothManager package
            // via the nuget dependency manager, creating an embedded server will
            // automatically add the UWPBluetoothManager to the Device Manager.

            // However, if for some reason you want to add another manager
            // outside of the automatic system, you can do so by passing a
            // closure to the AddDeviceSubtypeManager function. A closure is
            // required because we need to be able to populate the new manager
            // with the LogManager object, so we need a factory function to do so.

            // connector.Server.AddDeviceSubtypeManager((IButtplugLogManager aMgr)
            //   => { return new XInputManager(aMgr); });

            var client = new ButtplugClient("Example Client", connector);
        }
    }
}