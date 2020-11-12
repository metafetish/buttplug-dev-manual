using System;
using Buttplug.Client;
using Buttplug.Client.Connectors.WebsocketConnector;

namespace RemoteConnectorExample
{
    class Program
    {
        static void Main(string[] args)
        {
            // Creating a Websocket Connector is as easy as adding the
            // Client.Connectors.WebsocketConnector package from nuget and
            // passing a websocket address to it.
            var connector = new ButtplugWebsocketConnector(
                new Uri("ws://localhost:12345/buttplug"));
            var client = new ButtplugClient("Example Client", connector);
        }
    }
}
