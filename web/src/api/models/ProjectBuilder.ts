/* tslint:disable */
/* eslint-disable */
/**
 * Nine Yards REST API
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: rabbitminers@zohomail.eu
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { exists, mapValues } from '../runtime';
/**
 * 
 * @export
 * @interface ProjectBuilder
 */
export interface ProjectBuilder {
    /**
     * The project's icon's url
     * @type {string}
     * @memberof ProjectBuilder
     */
    iconUrl: string;
    /**
     * The project's name (3 -> 30 charachters)
     * @type {string}
     * @memberof ProjectBuilder
     */
    name: string;
    /**
     * The project's visibility,
     * @type {number}
     * @memberof ProjectBuilder
     */
    publicPermissions: number;
}

/**
 * Check if a given object implements the ProjectBuilder interface.
 */
export function instanceOfProjectBuilder(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "iconUrl" in value;
    isInstance = isInstance && "name" in value;
    isInstance = isInstance && "publicPermissions" in value;

    return isInstance;
}

export function ProjectBuilderFromJSON(json: any): ProjectBuilder {
    return ProjectBuilderFromJSONTyped(json, false);
}

export function ProjectBuilderFromJSONTyped(json: any, ignoreDiscriminator: boolean): ProjectBuilder {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'iconUrl': json['icon_url'],
        'name': json['name'],
        'publicPermissions': json['public_permissions'],
    };
}

export function ProjectBuilderToJSON(value?: ProjectBuilder | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'icon_url': value.iconUrl,
        'name': value.name,
        'public_permissions': value.publicPermissions,
    };
}

