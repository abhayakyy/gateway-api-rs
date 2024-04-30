// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --schema=derived --derive=JsonSchema --derive=Default --docs -f -
// kopium version: 0.17.2

use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Spec defines the desired state of TCPRoute.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1alpha2",
    kind = "TCPRoute",
    plural = "tcproutes"
)]
#[kube(namespaced)]
#[kube(status = "TCPRouteStatus")]
pub struct TCPRouteSpec {
    /// ParentRefs references the resources (usually Gateways) that a Route wants to be attached to. Note that the referenced parent resource needs to allow this for the attachment to be complete. For Gateways, that means the Gateway needs to allow attachment from Routes of this kind and namespace. For Services, that means the Service must either be in the same namespace for a "producer" route, or the mesh implementation must support and allow "consumer" routes for the referenced Service. ReferenceGrant is not applicable for governing ParentRefs to Services - it is not possible to create a "producer" route for a Service in a different namespace from the Route.
    ///  There are two kinds of parent resources with "Core" support:
    ///  * Gateway (Gateway conformance profile)  * Service (Mesh conformance profile, experimental, ClusterIP Services only)  This API may be extended in the future to support additional kinds of parent resources.
    ///  ParentRefs must be _distinct_. This means either that:
    ///  * They select different objects.  If this is the case, then parentRef entries are distinct. In terms of fields, this means that the multi-part key defined by `group`, `kind`, `namespace`, and `name` must be unique across all parentRef entries in the Route. * They do not select different objects, but for each optional field used, each ParentRef that selects the same object must set the same set of optional fields to different values. If one ParentRef sets a combination of optional fields, all must set the same combination.
    ///  Some examples:
    ///  * If one ParentRef sets `sectionName`, all ParentRefs referencing the same object must also set `sectionName`. * If one ParentRef sets `port`, all ParentRefs referencing the same object must also set `port`. * If one ParentRef sets `sectionName` and `port`, all ParentRefs referencing the same object must also set `sectionName` and `port`.
    ///  It is possible to separately reference multiple distinct objects that may be collapsed by an implementation. For example, some implementations may choose to merge compatible Gateway Listeners together. If that is the case, the list of routes attached to those resources should also be merged.
    ///  Note that for ParentRefs that cross namespace boundaries, there are specific rules. Cross-namespace references are only valid if they are explicitly allowed by something in the namespace they are referring to. For example, Gateway has the AllowedRoutes field, and ReferenceGrant provides a generic way to enable other kinds of cross-namespace reference.
    ///   ParentRefs from a Route to a Service in the same namespace are "producer" routes, which apply default routing rules to inbound connections from any namespace to the Service.
    ///  ParentRefs from a Route to a Service in a different namespace are "consumer" routes, and these routing rules are only applied to outbound connections originating from the same namespace as the Route, for which the intended destination of the connections are a Service targeted as a ParentRef of the Route.  
    ///  
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "parentRefs"
    )]
    pub parent_refs: Option<Vec<TCPRouteParentRefs>>,
    /// Rules are a list of TCP matchers and actions.
    pub rules: Vec<TCPRouteRules>,
}

/// ParentReference identifies an API object (usually a Gateway) that can be considered a parent of this resource (usually a route). There are two kinds of parent resources with "Core" support:
///  * Gateway (Gateway conformance profile) * Service (Mesh conformance profile, experimental, ClusterIP Services only)
///  This API may be extended in the future to support additional kinds of parent resources.
///  The API object must be valid in the cluster; the Group and Kind must be registered in the cluster for this reference to be valid.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
pub struct TCPRouteParentRefs {
    /// Group is the group of the referent. When unspecified, "gateway.networking.k8s.io" is inferred. To set the core API group (such as for a "Service" kind referent), Group must be explicitly set to "" (empty string).
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is kind of the referent.
    ///  There are two kinds of parent resources with "Core" support:
    ///  * Gateway (Gateway conformance profile) * Service (Mesh conformance profile, experimental, ClusterIP Services only)
    ///  Support for other resources is Implementation-Specific.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent.
    ///  Support: Core
    pub name: String,
    /// Namespace is the namespace of the referent. When unspecified, this refers to the local namespace of the Route.
    ///  Note that there are specific rules for ParentRefs which cross namespace boundaries. Cross-namespace references are only valid if they are explicitly allowed by something in the namespace they are referring to. For example: Gateway has the AllowedRoutes field, and ReferenceGrant provides a generic way to enable any other kind of cross-namespace reference.
    ///   ParentRefs from a Route to a Service in the same namespace are "producer" routes, which apply default routing rules to inbound connections from any namespace to the Service.
    ///  ParentRefs from a Route to a Service in a different namespace are "consumer" routes, and these routing rules are only applied to outbound connections originating from the same namespace as the Route, for which the intended destination of the connections are a Service targeted as a ParentRef of the Route.  
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Port is the network port this Route targets. It can be interpreted differently based on the type of parent resource.
    ///  When the parent resource is a Gateway, this targets all listeners listening on the specified port that also support this kind of Route(and select this Route). It's not recommended to set `Port` unless the networking behaviors specified in a Route must apply to a specific port as opposed to a listener(s) whose port(s) may be changed. When both Port and SectionName are specified, the name and port of the selected listener must match both specified values.
    ///   When the parent resource is a Service, this targets a specific port in the Service spec. When both Port (experimental) and SectionName are specified, the name and port of the selected port must match both specified values.  
    ///  Implementations MAY choose to support other parent resources. Implementations supporting other types of parent resources MUST clearly document how/if Port is interpreted.
    ///  For the purpose of status, an attachment is considered successful as long as the parent resource accepts it partially. For example, Gateway listeners can restrict which Routes can attach to them by Route kind, namespace, or hostname. If 1 of 2 Gateway listeners accept attachment from the referencing Route, the Route MUST be considered successfully attached. If no Gateway listeners accept attachment from this Route, the Route MUST be considered detached from the Gateway.
    ///  Support: Extended
    ///  
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// SectionName is the name of a section within the target resource. In the following resources, SectionName is interpreted as the following:
    ///  * Gateway: Listener Name. When both Port (experimental) and SectionName are specified, the name and port of the selected listener must match both specified values. * Service: Port Name. When both Port (experimental) and SectionName are specified, the name and port of the selected listener must match both specified values. Note that attaching Routes to Services as Parents is part of experimental Mesh support and is not supported for any other purpose.
    ///  Implementations MAY choose to support attaching Routes to other resources. If that is the case, they MUST clearly document how SectionName is interpreted.
    ///  When unspecified (empty string), this will reference the entire resource. For the purpose of status, an attachment is considered successful if at least one section in the parent resource accepts it. For example, Gateway listeners can restrict which Routes can attach to them by Route kind, namespace, or hostname. If 1 of 2 Gateway listeners accept attachment from the referencing Route, the Route MUST be considered successfully attached. If no Gateway listeners accept attachment from this Route, the Route MUST be considered detached from the Gateway.
    ///  Support: Core
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sectionName"
    )]
    pub section_name: Option<String>,
}

/// TCPRouteRule is the configuration for a given rule.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
pub struct TCPRouteRules {
    /// BackendRefs defines the backend(s) where matching requests should be sent. If unspecified or invalid (refers to a non-existent resource or a Service with no endpoints), the underlying implementation MUST actively reject connection attempts to this backend. Connection rejections must respect weight; if an invalid backend is requested to have 80% of connections, then 80% of connections must be rejected instead.
    ///  Support: Core for Kubernetes Service
    ///  Support: Extended for Kubernetes ServiceImport
    ///  Support: Implementation-specific for any other resource
    ///  Support for weight: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backendRefs"
    )]
    pub backend_refs: Option<Vec<TCPRouteRulesBackendRefs>>,
}

/// BackendRef defines how a Route should forward a request to a Kubernetes resource.
///  Note that when a namespace different than the local namespace is specified, a ReferenceGrant object is required in the referent namespace to allow that namespace's owner to accept the reference. See the ReferenceGrant documentation for details.
///  <gateway:experimental:description>
///  When the BackendRef points to a Kubernetes Service, implementations SHOULD honor the appProtocol field if it is set for the target Service Port.
///  Implementations supporting appProtocol SHOULD recognize the Kubernetes Standard Application Protocols defined in KEP-3726.
///  If a Service appProtocol isn't specified, an implementation MAY infer the backend protocol through its own means. Implementations MAY infer the protocol from the Route type referring to the backend Service.
///  If a Route is not able to send traffic to the backend using the specified protocol then the backend is considered invalid. Implementations MUST set the "ResolvedRefs" condition to "False" with the "UnsupportedProtocol" reason.
///  </gateway:experimental:description>
///  Note that when the BackendTLSPolicy object is enabled by the implementation, there are some extra rules about validity to consider here. See the fields where this struct is used for more information about the exact behavior.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
pub struct TCPRouteRulesBackendRefs {
    /// Group is the group of the referent. For example, "gateway.networking.k8s.io". When unspecified or empty string, core API group is inferred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is the Kubernetes resource kind of the referent. For example "Service".
    ///  Defaults to "Service" when not specified.
    ///  ExternalName services can refer to CNAME DNS records that may live outside of the cluster and as such are difficult to reason about in terms of conformance. They also may not be safe to forward to (see CVE-2021-25740 for more information). Implementations SHOULD NOT support ExternalName Services.
    ///  Support: Core (Services with a type other than ExternalName)
    ///  Support: Implementation-specific (Services with type ExternalName)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent.
    pub name: String,
    /// Namespace is the namespace of the backend. When unspecified, the local namespace is inferred.
    ///  Note that when a namespace different than the local namespace is specified, a ReferenceGrant object is required in the referent namespace to allow that namespace's owner to accept the reference. See the ReferenceGrant documentation for details.
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Port specifies the destination port number to use for this resource. Port is required when the referent is a Kubernetes Service. In this case, the port number is the service port number, not the target port. For other resources, destination port might be derived from the referent resource or this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Weight specifies the proportion of requests forwarded to the referenced backend. This is computed as weight/(sum of all weights in this BackendRefs list). For non-zero values, there may be some epsilon from the exact proportion defined here depending on the precision an implementation supports. Weight is not a percentage and the sum of weights does not need to equal 100.
    ///  If only one backend is specified and it has a weight greater than 0, 100% of the traffic is forwarded to that backend. If weight is set to 0, no traffic should be forwarded for this entry. If unspecified, weight defaults to 1.
    ///  Support for this field varies based on the context where used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

/// Status defines the current state of TCPRoute.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
pub struct TCPRouteStatus {
    /// Parents is a list of parent resources (usually Gateways) that are associated with the route, and the status of the route with respect to each parent. When this route attaches to a parent, the controller that manages the parent must add an entry to this list when the controller first sees the route and should update the entry as appropriate when the route or gateway is modified.
    ///  Note that parent references that cannot be resolved by an implementation of this API will not be added to this list. Implementations of this API can only populate Route status for the Gateways/parent resources they are responsible for.
    ///  A maximum of 32 Gateways will be represented in this list. An empty list means the route has not been attached to any Gateway.
    pub parents: Vec<TCPRouteStatusParents>,
}

/// RouteParentStatus describes the status of a route with respect to an associated Parent.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
pub struct TCPRouteStatusParents {
    /// Conditions describes the status of the route with respect to the Gateway. Note that the route's availability is also subject to the Gateway's own status conditions and listener status.
    ///  If the Route's ParentRef specifies an existing Gateway that supports Routes of this kind AND that Gateway's controller has sufficient access, then that Gateway's controller MUST set the "Accepted" condition on the Route, to indicate whether the route has been accepted or rejected by the Gateway, and why.
    ///  A Route MUST be considered "Accepted" if at least one of the Route's rules is implemented by the Gateway.
    ///  There are a number of cases where the "Accepted" condition may not be set due to lack of controller visibility, that includes when:
    ///  * The Route refers to a non-existent parent. * The Route is of a type that the controller does not support. * The Route is in a namespace the controller does not have access to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ControllerName is a domain/path string that indicates the name of the controller that wrote this status. This corresponds with the controllerName field on GatewayClass.
    ///  Example: "example.net/gateway-controller".
    ///  The format of this field is DOMAIN "/" PATH, where DOMAIN and PATH are valid Kubernetes names (https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names).
    ///  Controllers MUST populate this field when writing status. Controllers should ensure that entries to status populated with their ControllerName are cleaned up when they are no longer necessary.
    #[serde(rename = "controllerName")]
    pub controller_name: String,
    /// ParentRef corresponds with a ParentRef in the spec that this RouteParentStatus struct describes the status of.
    #[serde(rename = "parentRef")]
    pub parent_ref: TCPRouteStatusParentsParentRef,
}

/// ParentRef corresponds with a ParentRef in the spec that this RouteParentStatus struct describes the status of.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
pub struct TCPRouteStatusParentsParentRef {
    /// Group is the group of the referent. When unspecified, "gateway.networking.k8s.io" is inferred. To set the core API group (such as for a "Service" kind referent), Group must be explicitly set to "" (empty string).
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is kind of the referent.
    ///  There are two kinds of parent resources with "Core" support:
    ///  * Gateway (Gateway conformance profile) * Service (Mesh conformance profile, experimental, ClusterIP Services only)
    ///  Support for other resources is Implementation-Specific.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent.
    ///  Support: Core
    pub name: String,
    /// Namespace is the namespace of the referent. When unspecified, this refers to the local namespace of the Route.
    ///  Note that there are specific rules for ParentRefs which cross namespace boundaries. Cross-namespace references are only valid if they are explicitly allowed by something in the namespace they are referring to. For example: Gateway has the AllowedRoutes field, and ReferenceGrant provides a generic way to enable any other kind of cross-namespace reference.
    ///   ParentRefs from a Route to a Service in the same namespace are "producer" routes, which apply default routing rules to inbound connections from any namespace to the Service.
    ///  ParentRefs from a Route to a Service in a different namespace are "consumer" routes, and these routing rules are only applied to outbound connections originating from the same namespace as the Route, for which the intended destination of the connections are a Service targeted as a ParentRef of the Route.  
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Port is the network port this Route targets. It can be interpreted differently based on the type of parent resource.
    ///  When the parent resource is a Gateway, this targets all listeners listening on the specified port that also support this kind of Route(and select this Route). It's not recommended to set `Port` unless the networking behaviors specified in a Route must apply to a specific port as opposed to a listener(s) whose port(s) may be changed. When both Port and SectionName are specified, the name and port of the selected listener must match both specified values.
    ///   When the parent resource is a Service, this targets a specific port in the Service spec. When both Port (experimental) and SectionName are specified, the name and port of the selected port must match both specified values.  
    ///  Implementations MAY choose to support other parent resources. Implementations supporting other types of parent resources MUST clearly document how/if Port is interpreted.
    ///  For the purpose of status, an attachment is considered successful as long as the parent resource accepts it partially. For example, Gateway listeners can restrict which Routes can attach to them by Route kind, namespace, or hostname. If 1 of 2 Gateway listeners accept attachment from the referencing Route, the Route MUST be considered successfully attached. If no Gateway listeners accept attachment from this Route, the Route MUST be considered detached from the Gateway.
    ///  Support: Extended
    ///  
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// SectionName is the name of a section within the target resource. In the following resources, SectionName is interpreted as the following:
    ///  * Gateway: Listener Name. When both Port (experimental) and SectionName are specified, the name and port of the selected listener must match both specified values. * Service: Port Name. When both Port (experimental) and SectionName are specified, the name and port of the selected listener must match both specified values. Note that attaching Routes to Services as Parents is part of experimental Mesh support and is not supported for any other purpose.
    ///  Implementations MAY choose to support attaching Routes to other resources. If that is the case, they MUST clearly document how SectionName is interpreted.
    ///  When unspecified (empty string), this will reference the entire resource. For the purpose of status, an attachment is considered successful if at least one section in the parent resource accepts it. For example, Gateway listeners can restrict which Routes can attach to them by Route kind, namespace, or hostname. If 1 of 2 Gateway listeners accept attachment from the referencing Route, the Route MUST be considered successfully attached. If no Gateway listeners accept attachment from this Route, the Route MUST be considered detached from the Gateway.
    ///  Support: Core
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sectionName"
    )]
    pub section_name: Option<String>,
}