//! Models for ClickOnce deployment manifest
//!
use crate::FromXML;
use serde::Deserialize;

fn default_win32() -> String {
    "win32".to_string()
}

fn default_false() -> bool {
    false
}

/// Model for ClickOnce deployment manifest
///
/// Defined at https://docs.microsoft.com/en-us/visualstudio/deployment/clickonce-deployment-manifest?view=vs-2022
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct DeploymentManifest {
    /// Required. The top-level element for the deployment manifest.
    pub assembly: Assembly,
}

impl FromXML<'_> for DeploymentManifest {}

/// Model for <assembly> element
///
/// Defined at https://docs.microsoft.com/en-us/visualstudio/deployment/assembly-element-clickonce-deployment?view=vs-2022
#[derive(Debug, Deserialize)]
pub struct Assembly {
    /// Attribute. Required. This attribute must be set to 1.0.
    #[serde(alias = "manifestVersion")]
    pub manifest_version: String,

    /// Child. Required. Identifies the application manifest for the ClickOnce application.
    #[serde(alias = "assemblyIdentity")]
    pub assembly_identity: AssemblyIdentity,

    /// Child. Required.  Identifies application information used to create a shell presence and the Add or Remove Programs item in Control Panel.
    pub description: Description,

    /// Child. Optional. Identifies the attributes used for the deployment of updates and exposure to the system.
    pub deployment: Option<Deployment>,

    /// Child. Required. Identifies the versions of the .NET Framework where this application can install and run.
    #[serde(alias = "compatibleFrameworks")]
    pub compatible_frameworks: CompatibleFrameworks,

    /// Child. Required. Identifies the version of the application to install for the deployment and the location of the application manifest.
    pub dependency: Vec<Dependency>,
}

/// Model for <assemblyIdentity> element
///
/// Defined at https://docs.microsoft.com/en-us/visualstudio/deployment/assemblyidentity-element-clickonce-deployment?view=vs-2022
#[derive(Debug, Deserialize)]
pub struct AssemblyIdentity {
    /// Attribute. Required. Identifies the human-readable name of the deployment for informational purposes.
    ///
    /// If `name` contains special characters, such as single or double quotes, the application may fail to activate.
    pub name: String,

    /// Attribute. Required. Specifies the version number of the assembly, in the following format: major.minor.build.revision.
    ///
    /// This value must be incremented in an updated manifest to trigger an application update.
    pub version: String,

    /// Attribute. Required.Specifies a 16-character hexadecimal string that represents the last 8 bytes of the SHA-1 hash value of the public key under which the deployment manifest is signed. The public key that is used to sign must be 2048 bits or greater.
    ///
    /// Although signing an assembly is recommended but optional, this attribute is required. If an assembly is unsigned, you should copy a value from a self-signed assembly or use a "dummy" value of all zeros.
    #[serde(alias = "publicKeyToken")]
    pub public_key_token: String,

    /// Attribute. Required. Specifies the processor. The valid values are msil for all processors, x86 for 32-bit Windows, IA64 for 64-bit Windows, and Itanium for Intel 64-bit Itanium processors.
    #[serde(alias = "processorArchitecture")]
    pub processor_architecture: ProcessorArchitecture,

    /// Attribute. Required. For compatibility with Windows side-by-side installation technology. The only allowed value is win32.
    #[serde(alias = "type")]
    #[serde(default = "default_win32")]
    pub r#type: String,
}

/// Types of permitted processor architecture
#[derive(Debug, Deserialize)]
pub enum ProcessorArchitecture {
    /// All processors
    #[serde(alias = "msil")]
    Msil,
    /// 32-bit Windows
    #[serde(alias = "x86")]
    X86,
    /// 64-bit Windows
    IA64,
    /// Intel 64-bit Itanium processors
    Itanium,
}

/// Model for <Description> element
///
/// Defined at https://docs.microsoft.com/en-us/visualstudio/deployment/description-element-clickonce-deployment?view=vs-2022
#[derive(Debug, Deserialize)]
pub struct Description {
    /// Attribute. Required. Identifies the company name used for icon placement in the Windows Start menu and the Add or Remove Programs item in Control Panel, when the deployment is configured for install.
    pub publisher: String,

    /// Attribute. Required. Identifies the full product name. Used as the title for the icon installed in the Windows Start menu.
    pub product: String,

    /// Attribute. Optional. Identifies a subfolder within the publisher folder in the Windows Start menu.
    #[serde(alias = "suiteName")]
    pub suite_name: Option<String>,

    /// Attribute. Optional. Specifies a support URL that is shown in the Add or Remove Programs item in Control Panel. A shortcut to this URL is also created for application support in the Windows Start menu, when the deployment is configured for installation.
    #[serde(alias = "supportUrl")]
    pub support_url: Option<String>,
}

/// Model for <Deployment> element
///
/// Defined at https://docs.microsoft.com/en-us/visualstudio/deployment/deployment-element-clickonce-deployment?view=vs-2022
#[derive(Debug, Deserialize)]
pub struct Deployment {
    /// Attribute. Required. Specifies whether this application defines a presence on the Windows Start menu
    /// and in the Control Panel Add or Remove Programs application. Valid values are true and false. If
    /// false, ClickOnce will always run the latest version of this application from the network, and will
    /// not recognize the subscription element.
    pub install: bool,

    /// Attribute. Optional. Specifies the minimum version of this application that can run on the client.
    /// If the version number of the application is less than the version number supplied in the deployment
    /// manifest, the application will not run. Version numbers must be specified in the format N.N.N.N,
    /// where N is an unsigned integer. If the install attribute is false, minimumRequiredVersion must not
    /// be set.
    #[serde(alias = "minimumRequiredVersion")]
    pub minimum_required_version: Option<String>,

    /// Attribute. Optional. Defaults to false. If true, all files in the deployment must have a .deploy
    /// extension. ClickOnce will strip this extension off these files as soon as it downloads them from
    /// the Web server. If you publish your application by using Visual Studio, it automatically adds this
    /// extension to all files. This parameter allows all the files within a ClickOnce deployment to be
    /// downloaded from a Web server that blocks transmission of files ending in "unsafe" extensions such
    /// as .exe.
    #[serde(alias = "mapFileExtensions")]
    #[serde(default = "default_false")]
    pub map_file_extensions: bool,

    /// Attribute. Optional. Defaults to false. If true, prevents an installed application from being
    /// started by clicking the URL or entering the URL into Internet Explorer. If the install attribute
    /// is not present, this attribute is ignored.
    #[serde(alias = "disallowUrlActivation")]
    #[serde(default = "default_false")]
    pub disallow_url_activation: bool,

    /// Attribute. Optional. Defaults to false. If true, allows the URL to contain query string
    /// parameters that are passed into the application, much like command-line arguments are passed
    /// to a command-line application. For more information, see How to: Retrieve Query String
    /// Information in an Online ClickOnce Application.
    #[serde(alias = "trustURLParameters")]
    #[serde(default = "default_false")]
    pub trust_url_parameters: bool,

    /// Child. Optional. Contains the update element. The subscription element has no attributes.
    /// If the subscription element does not exist, the ClickOnce application will never scan for
    /// updates. If the install attribute of the deployment element is false, the subscription element
    /// is ignored, because a ClickOnce application that is launched from the network always uses the
    /// latest version.
    pub subscription: Subscription,

    /// Child. For the .NET Framework 2.0, this element is required if the deployment manifest
    /// contains a subscription section. For the .NET Framework 3.5 and later, this element is optional,
    /// and will default to the server and file path in which the deployment manifest was discovered.
    #[serde(alias = "deploymentProvider")]
    pub deployment_provider: Option<DeploymentProvider>,
}

/// Optional. Contains the update element. The subscription element has no attributes. If the
/// subscription element does not exist, the ClickOnce application will never scan for updates. If the
/// install attribute of the deployment element is false, the subscription element is ignored, because
/// a ClickOnce application that is launched from the network always uses the latest version.
#[derive(Debug, Deserialize)]
pub struct Subscription {
    /// Required. This element is a child of the subscription element and contains either the
    /// beforeApplicationStartup or the expiration element. beforeApplicationStartup and expiration
    /// cannot both be specified in the same deployment manifest.
    pub update: Update,
}

/// Required. This element is a child of the subscription element and contains either the
/// beforeApplicationStartup or the expiration element. beforeApplicationStartup and expiration cannot
/// both be specified in the same deployment manifest.
#[derive(Debug, Deserialize)]
pub enum Update {
    /// No child found
    None,
    /// Child. Optional. This element is a child of the update element and has no attributes. When the
    /// beforeApplicationStartup element exists, the application will be blocked when ClickOnce checks
    /// for updates, if the client is online. If this element does not exist, ClickOnce will first scan
    /// for updates based on the values specified for the expiration element. beforeApplicationStartup
    /// and expiration cannot both be specified in the same deployment manifest.
    #[serde(alias = "beforeApplicationStartup")]
    BeforeApplicationStartup,

    /// Child. Optional. This element is a child of the update element, and has no children.
    /// beforeApplicationStartup and expiration cannot both be specified in the same deployment manifest. When the update check occurs and an updated version is detected, the new version caches while the existing version runs. The new version then installs on the next launch of the ClickOnce application.
    #[serde(alias = "expiration")]
    Expiration(Expiration),
}

/// Optional. This element is a child of the update element, and has no children. beforeApplicationStartup and expiration cannot both be specified in the same deployment manifest. When the update check occurs and an updated version is detected, the new version caches while the existing version runs. The new version then installs on the next launch of the ClickOnce application.
#[derive(Debug, Deserialize)]
pub struct Expiration {
    /// Attribute. Required. Identifies how old the current update should become before the application performs an update check. The unit of time is determined by the unit attribute.
    #[serde(alias = "maximumAge")]
    pub maximum_age: String,
    /// Attribute. Required. Identifies the unit of time for maximumAge. Valid units are hours, days, and weeks.
    pub unit: String,
}

/// For the .NET Framework 2.0, this element is required if the deployment manifest contains a subscription section. For the .NET Framework 3.5 and later, this element is optional, and will default to the server and file path in which the deployment manifest was discovered.
#[derive(Debug, Deserialize)]
pub struct DeploymentProvider {
    /// Attribute. Required. Identifies the location, as a Uniform Resource Identifier (URI), of the deployment manifest that is used to update the ClickOnce application. This element also allows for forwarding update locations for CD-based installations. Must be a valid URI.
    pub codebase: String,
}

/// Model for <compatibleFrameworks> element
///
/// Defined at https://docs.microsoft.com/en-us/visualstudio/deployment/compatibleframeworks-element-clickonce-deployment?view=vs-2022
///
/// The compatibleFrameworks element is required for deployment manifests that target the ClickOnce runtime provided by .NET Framework 4 or later. The compatibleFrameworks element contains one or more framework elements that specify the .NET Framework versions on which this application can run. The ClickOnce runtime will run the application on the first available framework in this list.
#[derive(Debug, Deserialize)]
pub struct CompatibleFrameworks {
    /// Attribute. Optional. Specifies a URL where the preferred compatible .NET Framework version can be downloaded.
    pub support_url: Option<String>,

    /// Children.
    #[serde(alias = "framework")]
    pub frameworks: Vec<Framework>,
}

/// Structure covering the Framework
#[derive(Debug, Deserialize)]
pub struct Framework {
    /// Attribute. Required. Specifies the version number of the target .NET Framework.
    #[serde(alias = "targetVersion")]
    pub target_version: String,
    /// Attribute. Required. Specifies the profile of the target .NET Framework.
    pub profile: String,
    /// Attribute. Required. Specifies the version number of the runtime associated with the target .NET Framework.
    #[serde(alias = "supportedRuntime")]
    pub supported_runtime: String,
}

/// Model for <Dependency> element
///
/// Defined at https://docs.microsoft.com/en-us/visualstudio/deployment/dependency-element-clickonce-deployment?view=vs-2022
///
/// Identifies the version of the application to install for the deployment and the location of the application manifest.
#[derive(Debug, Deserialize)]
pub struct Dependency {
    /// Child. Parent of assemblyIdentity.
    #[serde(alias = "dependentAssembly")]
    pub dependent_assembly: DependentAssembly,
}

/// Parent of assemblyIdentity.
#[derive(Debug, Deserialize)]
pub struct DependentAssembly {
    /// Attribute. Optional. Specifies that this assembly should already exist in the GAC. Valid values are true and false. If true, and the specified assembly does not exist in the GAC, the application fails to run.
    #[serde(alias = "preRequisite")]
    pub prerequisite: Option<bool>,

    /// Attribute. Optional. Identifies the top-level application identity, including its dependencies. Used internally by ClickOnce to manage application storage and activation.
    pub visible: Option<String>,

    /// Attribute. Required. The relationship between this dependency and the application. Valid values are:
    ///
    /// - install. Component represents a separate installation from the current application.
    /// - preRequisite. Component is required by the current application.
    #[serde(alias = "dependencyType")]
    pub dependency_type: DependencyType,

    /// Attribute. Optional. The full path to the application manifest.
    pub codebase: Option<String>,

    /// Attribute. Optional. The size of the application manifest, in bytes.
    pub size: u32,

    /// Child. Required. Content should be the same as the application manifest.
    #[serde(alias = "assemblyIdentity")]
    pub assembly_identity: AssemblyIdentity,

    /// Child. Optional. ClickOnce uses an algorithmic hash of all the files in an application as a security check to ensure that none of the files were changed after deployment. If the hash element is not included, this check will not be performed. Therefore, omitting the hash element is not recommended.
    pub hash: Option<Hash>,
}

/// Types for the relationship between a dependency and the application
#[derive(Debug, Deserialize)]
pub enum DependencyType {
    /// Component represents a separate installation from the current application.
    #[serde(alias = "install")]
    Install,

    /// Component is required by the current application.
    #[serde(alias = "preRequisite")]
    Prerequisite,
}

/// Model for <hash>
#[derive(Debug, Deserialize)]
pub struct Hash {
    /// Child. Required. Parent of dsig:Transform
    #[serde(alias = "Transforms")]
    pub dsig_transforms: DsigTransforms,

    /// Child. Required.
    #[serde(alias = "DigestMethod")]
    pub dsig_digest_method: DsigDigestMethod,

    /// Child. Required.
    #[serde(alias = "DigestValue")]
    pub dsig_digest_value: DsigDigestValue,
}
/// Model for <dsig:Transforms>
#[derive(Debug, Deserialize)]
pub struct DsigTransforms {
    /// Child. Required. dsig:Transform
    #[serde(alias = "Transform")]
    pub dsig_transform: DsigTransform,
}
/// Model for <dsig:Transform>
#[derive(Debug, Deserialize)]
pub struct DsigTransform {
    /// Attribute. Required. The algorithm used to calculate the digest for this file. Currently the only value used by ClickOnce is urn:schemas-microsoft-com:HashTransforms.Identity.
    #[serde(alias = "Algorithm")]
    pub algorithm: String,
}
/// Model for <dsig:DigestMethod>
#[derive(Debug, Deserialize)]
pub struct DsigDigestMethod {
    /// Attribute. Required. The algorithm used to calculate the digest for this file. Currently the only value used by ClickOnce is http://www.w3.org/2000/09/xmldsig#sha1.
    #[serde(alias = "Algorithm")]
    pub algorithm: String,
}
/// Model for <dsig:DigestValue>
#[derive(Debug, Deserialize)]
pub struct DsigDigestValue {
    /// Text. Required.
    #[serde(rename = "$value")]
    pub value: String,
}

/// TODO: Parse the publisherIdentity, Signature and customErrorReporting elements

#[cfg(test)]
mod tests {
    use super::*;

    fn deployment_manifest() -> &'static str {
        include_str!("example/sample.application").trim_start_matches('\u{feff}')
    }

    #[test]
    fn parse_manifest() {
        let manifest_contents = deployment_manifest();
        let manifest =
            DeploymentManifest::from_xml(manifest_contents).expect("Failure parsing manifest");
        println!("Manifest: {:#?}", manifest);
        assert_eq!(manifest.assembly.assembly_identity.r#type, "win32");
    }
}
