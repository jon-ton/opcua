// This file was autogenerated by tools/schema/gen_supported_message.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
use node_id::NodeId;
use basic_types::UInt32;
use service_types::*;
use node_ids::ObjectId;
use tcp_types::AcknowledgeMessage;

/// This macro helps avoid tedious repetition as new messages are added
/// The first form just handles the trailing comma after the last entry to save some pointless
/// editing when new messages are added to the list.
macro_rules! supported_messages_enum {
    [ $( $x:ident, ) * ] => (supported_messages_enum![ $( $x ),* ];);
    [ $( $x:ident ), * ] => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum SupportedMessage {
            /// An invalid request / response of some form
            Invalid(ObjectId),
            /// Acknowledge message
            AcknowledgeMessage(AcknowledgeMessage),
            /// Other messages
            $( $x($x), )*
        }

        impl BinaryEncoder <SupportedMessage> for SupportedMessage {
            fn byte_len(&self) -> usize {
                match *self {
                    SupportedMessage::Invalid(object_id) => {
                        panic!("Unsupported message byte_len {:?}", object_id);
                    },
                    SupportedMessage::AcknowledgeMessage(ref value) => value.byte_len(),
                    $( SupportedMessage::$x(ref value) => value.byte_len(), )*
                }
            }

            fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
                match *self {
                    SupportedMessage::Invalid(object_id) => {
                        panic!("Unsupported message encode {:?}", object_id);
                    },
                    SupportedMessage::AcknowledgeMessage(ref value) => value.encode(stream),
                    $( SupportedMessage::$x(ref value) => value.encode(stream), )*
                }
            }

            fn decode<S: Read>(_: &mut S) -> EncodingResult<Self> {
                // THIS WILL NOT DO ANYTHING
                panic!("Cannot decode a stream to a supported message type");
            }
        }
        
        $(
        impl Into<SupportedMessage> for $x {
            fn into(self) -> SupportedMessage { SupportedMessage::$x(self) }
        }
        )*

        impl SupportedMessage {
            pub fn node_id(&self) -> NodeId {
                match *self {
                    SupportedMessage::Invalid(object_id) => {
                        panic!("Unsupported message invalid, node_id {:?}", object_id);
                    },
                    SupportedMessage::AcknowledgeMessage(ref value) => {
                        panic!("Unsupported message node_id {:?}", value);
                    },
                    $( SupportedMessage::$x(ref value) => value.object_id().into(), )*
                }
            }
        }
    }
}

impl SupportedMessage {
    pub fn request_handle(&self) -> UInt32 {
        match *self {
            SupportedMessage::Invalid(_) | SupportedMessage::AcknowledgeMessage(_) => 0,
            // Requests
            SupportedMessage::OpenSecureChannelRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::CloseSecureChannelRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::GetEndpointsRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::FindServersRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::RegisterServerRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::CreateSessionRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::CloseSessionRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::ActivateSessionRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::CreateMonitoredItemsRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::ModifyMonitoredItemsRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::DeleteMonitoredItemsRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::CreateSubscriptionRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::ModifySubscriptionRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::DeleteSubscriptionsRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::SetPublishingModeRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::BrowseRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::BrowseNextRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::PublishRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::RepublishRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::TranslateBrowsePathsToNodeIdsRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::ReadRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::WriteRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::CallRequest(ref r) => r.request_header.request_handle,
            // Responses
            SupportedMessage::ServiceFault(ref r) => r.response_header.request_handle,
            SupportedMessage::OpenSecureChannelResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::CloseSecureChannelResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::GetEndpointsResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::FindServersResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::RegisterServerResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::CreateSessionResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::CloseSessionResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::ActivateSessionResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::CreateMonitoredItemsResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::ModifyMonitoredItemsResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::DeleteMonitoredItemsResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::CreateSubscriptionResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::ModifySubscriptionResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::DeleteSubscriptionsResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::SetPublishingModeResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::BrowseResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::BrowseNextResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::PublishResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::RepublishResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::TranslateBrowsePathsToNodeIdsResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::ReadResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::WriteResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::CallResponse(ref r) => r.response_header.request_handle,
        }
    }

    pub fn decode_by_object_id<S: Read>(stream: &mut S, object_id: ObjectId) -> EncodingResult<Self> {
        trace!("decoding object_id {:?}", object_id);
        let decoded_message = match object_id {
            ObjectId::ServiceFault_Encoding_DefaultBinary => {
                SupportedMessage::ServiceFault(ServiceFault::decode(stream)?)
            }
            ObjectId::OpenSecureChannelRequest_Encoding_DefaultBinary => {
                SupportedMessage::OpenSecureChannelRequest(OpenSecureChannelRequest::decode(stream)?)
            }
            ObjectId::OpenSecureChannelResponse_Encoding_DefaultBinary => {
                SupportedMessage::OpenSecureChannelResponse(OpenSecureChannelResponse::decode(stream)?)
            }
            ObjectId::CloseSecureChannelRequest_Encoding_DefaultBinary => {
                SupportedMessage::CloseSecureChannelRequest(CloseSecureChannelRequest::decode(stream)?)
            }
            ObjectId::CloseSecureChannelResponse_Encoding_DefaultBinary => {
                SupportedMessage::CloseSecureChannelResponse(CloseSecureChannelResponse::decode(stream)?)
            }
            ObjectId::GetEndpointsRequest_Encoding_DefaultBinary => {
                SupportedMessage::GetEndpointsRequest(GetEndpointsRequest::decode(stream)?)
            }
            ObjectId::GetEndpointsResponse_Encoding_DefaultBinary => {
                SupportedMessage::GetEndpointsResponse(GetEndpointsResponse::decode(stream)?)
            }
            ObjectId::FindServersRequest_Encoding_DefaultBinary => {
                SupportedMessage::FindServersRequest(FindServersRequest::decode(stream)?)
            }
            ObjectId::FindServersResponse_Encoding_DefaultBinary => {
                SupportedMessage::FindServersResponse(FindServersResponse::decode(stream)?)
            }
            ObjectId::RegisterServerRequest_Encoding_DefaultBinary => {
                SupportedMessage::RegisterServerRequest(RegisterServerRequest::decode(stream)?)
            }
            ObjectId::RegisterServerResponse_Encoding_DefaultBinary => {
                SupportedMessage::RegisterServerResponse(RegisterServerResponse::decode(stream)?)
            }
            ObjectId::CreateSessionRequest_Encoding_DefaultBinary => {
                SupportedMessage::CreateSessionRequest(CreateSessionRequest::decode(stream)?)
            }
            ObjectId::CreateSessionResponse_Encoding_DefaultBinary => {
                SupportedMessage::CreateSessionResponse(CreateSessionResponse::decode(stream)?)
            }
            ObjectId::CloseSessionRequest_Encoding_DefaultBinary => {
                SupportedMessage::CloseSessionRequest(CloseSessionRequest::decode(stream)?)
            }
            ObjectId::CloseSessionResponse_Encoding_DefaultBinary => {
                SupportedMessage::CloseSessionResponse(CloseSessionResponse::decode(stream)?)
            }
            ObjectId::ActivateSessionRequest_Encoding_DefaultBinary => {
                SupportedMessage::ActivateSessionRequest(ActivateSessionRequest::decode(stream)?)
            }
            ObjectId::ActivateSessionResponse_Encoding_DefaultBinary => {
                SupportedMessage::ActivateSessionResponse(ActivateSessionResponse::decode(stream)?)
            }
            ObjectId::CreateMonitoredItemsRequest_Encoding_DefaultBinary => {
                SupportedMessage::CreateMonitoredItemsRequest(CreateMonitoredItemsRequest::decode(stream)?)
            }
            ObjectId::CreateMonitoredItemsResponse_Encoding_DefaultBinary => {
                SupportedMessage::CreateMonitoredItemsResponse(CreateMonitoredItemsResponse::decode(stream)?)
            }
            ObjectId::ModifyMonitoredItemsRequest_Encoding_DefaultBinary => {
                SupportedMessage::ModifyMonitoredItemsRequest(ModifyMonitoredItemsRequest::decode(stream)?)
            }
            ObjectId::ModifyMonitoredItemsResponse_Encoding_DefaultBinary => {
                SupportedMessage::ModifyMonitoredItemsResponse(ModifyMonitoredItemsResponse::decode(stream)?)
            }
            ObjectId::DeleteMonitoredItemsRequest_Encoding_DefaultBinary => {
                SupportedMessage::DeleteMonitoredItemsRequest(DeleteMonitoredItemsRequest::decode(stream)?)
            }
            ObjectId::DeleteMonitoredItemsResponse_Encoding_DefaultBinary => {
                SupportedMessage::DeleteMonitoredItemsResponse(DeleteMonitoredItemsResponse::decode(stream)?)
            }
            ObjectId::CreateSubscriptionRequest_Encoding_DefaultBinary => {
                SupportedMessage::CreateSubscriptionRequest(CreateSubscriptionRequest::decode(stream)?)
            }
            ObjectId::CreateSubscriptionResponse_Encoding_DefaultBinary => {
                SupportedMessage::CreateSubscriptionResponse(CreateSubscriptionResponse::decode(stream)?)
            }
            ObjectId::ModifySubscriptionRequest_Encoding_DefaultBinary => {
                SupportedMessage::ModifySubscriptionRequest(ModifySubscriptionRequest::decode(stream)?)
            }
            ObjectId::ModifySubscriptionResponse_Encoding_DefaultBinary => {
                SupportedMessage::ModifySubscriptionResponse(ModifySubscriptionResponse::decode(stream)?)
            }
            ObjectId::DeleteSubscriptionsRequest_Encoding_DefaultBinary => {
                SupportedMessage::DeleteSubscriptionsRequest(DeleteSubscriptionsRequest::decode(stream)?)
            }
            ObjectId::DeleteSubscriptionsResponse_Encoding_DefaultBinary => {
                SupportedMessage::DeleteSubscriptionsResponse(DeleteSubscriptionsResponse::decode(stream)?)
            }
            ObjectId::SetPublishingModeRequest_Encoding_DefaultBinary => {
                SupportedMessage::SetPublishingModeRequest(SetPublishingModeRequest::decode(stream)?)
            }
            ObjectId::SetPublishingModeResponse_Encoding_DefaultBinary => {
                SupportedMessage::SetPublishingModeResponse(SetPublishingModeResponse::decode(stream)?)
            }
            ObjectId::BrowseRequest_Encoding_DefaultBinary => {
                SupportedMessage::BrowseRequest(BrowseRequest::decode(stream)?)
            }
            ObjectId::BrowseResponse_Encoding_DefaultBinary => {
                SupportedMessage::BrowseResponse(BrowseResponse::decode(stream)?)
            }
            ObjectId::BrowseNextRequest_Encoding_DefaultBinary => {
                SupportedMessage::BrowseNextRequest(BrowseNextRequest::decode(stream)?)
            }
            ObjectId::BrowseNextResponse_Encoding_DefaultBinary => {
                SupportedMessage::BrowseNextResponse(BrowseNextResponse::decode(stream)?)
            }
            ObjectId::PublishRequest_Encoding_DefaultBinary => {
                SupportedMessage::PublishRequest(PublishRequest::decode(stream)?)
            }
            ObjectId::PublishResponse_Encoding_DefaultBinary => {
                SupportedMessage::PublishResponse(PublishResponse::decode(stream)?)
            }
            ObjectId::RepublishRequest_Encoding_DefaultBinary => {
                SupportedMessage::RepublishRequest(RepublishRequest::decode(stream)?)
            }
            ObjectId::RepublishResponse_Encoding_DefaultBinary => {
                SupportedMessage::RepublishResponse(RepublishResponse::decode(stream)?)
            }
            ObjectId::TranslateBrowsePathsToNodeIdsRequest_Encoding_DefaultBinary => {
                SupportedMessage::TranslateBrowsePathsToNodeIdsRequest(TranslateBrowsePathsToNodeIdsRequest::decode(stream)?)
            }
            ObjectId::TranslateBrowsePathsToNodeIdsResponse_Encoding_DefaultBinary => {
                SupportedMessage::TranslateBrowsePathsToNodeIdsResponse(TranslateBrowsePathsToNodeIdsResponse::decode(stream)?)
            }
            ObjectId::ReadRequest_Encoding_DefaultBinary => {
                SupportedMessage::ReadRequest(ReadRequest::decode(stream)?)
            }
            ObjectId::ReadResponse_Encoding_DefaultBinary => {
                SupportedMessage::ReadResponse(ReadResponse::decode(stream)?)
            }
            ObjectId::WriteRequest_Encoding_DefaultBinary => {
                SupportedMessage::WriteRequest(WriteRequest::decode(stream)?)
            }
            ObjectId::WriteResponse_Encoding_DefaultBinary => {
                SupportedMessage::WriteResponse(WriteResponse::decode(stream)?)
            }
            ObjectId::CallRequest_Encoding_DefaultBinary => {
                SupportedMessage::CallRequest(CallRequest::decode(stream)?)
            }
            ObjectId::CallResponse_Encoding_DefaultBinary => {
                SupportedMessage::CallResponse(CallResponse::decode(stream)?)
            }

            _ => {
                debug!("decoding unsupported for object id {:?}", object_id);
                SupportedMessage::Invalid(object_id)
            }
        };
        Ok(decoded_message)
    }
}

// These are all the messages handled into and out of streams by the OPCUA server / client code
supported_messages_enum![
    // Requests
    OpenSecureChannelRequest,
    CloseSecureChannelRequest,
    GetEndpointsRequest,
    FindServersRequest,
    RegisterServerRequest,
    CreateSessionRequest,
    CloseSessionRequest,
    ActivateSessionRequest,
    CreateMonitoredItemsRequest,
    ModifyMonitoredItemsRequest,
    DeleteMonitoredItemsRequest,
    CreateSubscriptionRequest,
    ModifySubscriptionRequest,
    DeleteSubscriptionsRequest,
    SetPublishingModeRequest,
    BrowseRequest,
    BrowseNextRequest,
    PublishRequest,
    RepublishRequest,
    TranslateBrowsePathsToNodeIdsRequest,
    ReadRequest,
    WriteRequest,
    CallRequest,
    // Responses
    ServiceFault,
    OpenSecureChannelResponse,
    CloseSecureChannelResponse,
    GetEndpointsResponse,
    FindServersResponse,
    RegisterServerResponse,
    CreateSessionResponse,
    CloseSessionResponse,
    ActivateSessionResponse,
    CreateMonitoredItemsResponse,
    ModifyMonitoredItemsResponse,
    DeleteMonitoredItemsResponse,
    CreateSubscriptionResponse,
    ModifySubscriptionResponse,
    DeleteSubscriptionsResponse,
    SetPublishingModeResponse,
    BrowseResponse,
    BrowseNextResponse,
    PublishResponse,
    RepublishResponse,
    TranslateBrowsePathsToNodeIdsResponse,
    ReadResponse,
    WriteResponse,
    CallResponse,
];
