//! Necrovalley
use super::prelude::*;

pub const ID: u32 = 47355498;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GRAVEKEEPERS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // All "Gravekeeper's" monsters on the field gain 500 ATK/DEF
    let e1a = effect::create_effect(d, c);
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1a, LOCATION_FZONE);
    effect::set_target_range(d, e1a, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e1a, Some(effect_card_fn(gk_target)));
    effect::set_value(d, e1a, 500);
    card::register_effect(d, c, e1a, false);
    let e1b = effect::clone(d, e1a);
    effect::set_code(d, e1b, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e1b, false);
    // Cards in the GY cannot be banished
    let e2a = effect::create_effect(d, c);
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2a, EFFECT_CANNOT_REMOVE);
    effect::set_range(d, e2a, LOCATION_FZONE);
    effect::set_target_range(d, e2a, LOCATION_GRAVE, 0);
    effect::set_condition(d, e2a, Some(condition(e2a_cond)));
    card::register_effect(d, c, e2a, false);
    let e2b = effect::clone(d, e2a);
    effect::set_target_range(d, e2b, 0, LOCATION_GRAVE);
    effect::set_condition(d, e2b, Some(condition(e2b_cond)));
    card::register_effect(d, c, e2b, false);
    // Negate any card effect that would move a card in the GY to a different place. Negate any card effect that changes Types or Attributes in the GY
    let e3a = effect::clone(d, e2a);
    effect::set_code(d, e3a, EFFECT_NECRO_VALLEY);
    card::register_effect(d, c, e3a, false);
    let e3b = effect::clone(d, e2b);
    effect::set_code(d, e3b, EFFECT_NECRO_VALLEY);
    card::register_effect(d, c, e3b, false);
    let e3c = effect::clone(d, e3a);
    effect::set_property(d, e3c, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e3c, 1, 0);
    card::register_effect(d, c, e3c, false);
    let e3d = effect::clone(d, e3b);
    effect::set_property(d, e3d, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e3d, 0, 1);
    card::register_effect(d, c, e3d, false);
    // Negate an activated effect when it resolves if it would move a card in the GY
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e4, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e4, LOCATION_FZONE);
    effect::set_operation(d, e4, Some(operation(disop)));
    card::register_effect(d, c, e4, false);
    // Prevent non-activated effects from Special Summoning from the GY (e.g. unclassified summoning effects or delayed effects)
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    effect::set_property(d, e5, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e5, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_range(d, e5, LOCATION_FZONE);
    effect::set_target_range(d, e5, 1, 1);
    effect::set_target(d, e5, Some(lua_fn(cannotsptg)));
    card::register_effect(d, c, e5, false);
}
/// `function(e,c) return c:IsSetCard(SET_GRAVEKEEPERS) end`
fn gk_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_GRAVEKEEPERS as u16])
}
fn e2a_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsPlayerAffectedByEffect(e:GetHandler():GetControler(),EFFECT_NECRO_VALLEY_IM)
    let h = effect::get_handler(d, a.e).nil();
    let p = card::get_controler(d, h);
    duel::is_player_affected_by_effect(d, p, EFFECT_NECRO_VALLEY_IM).is_none()
}
fn e2b_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsPlayerAffectedByEffect(1-e:GetHandler():GetControler(),EFFECT_NECRO_VALLEY_IM)
    let h = effect::get_handler(d, a.e).nil();
    let p = 1 - card::get_controler(d, h);
    duel::is_player_affected_by_effect(d, p, EFFECT_NECRO_VALLEY_IM).is_none()
}
/// `s.discheckfilter(c,not_im0,not_im1,re)`
fn discheckfilter(d: &mut Duel, c: CardId, not_im0: bool, not_im1: bool, re: EffectId) -> bool {
    // if c:IsControler(0) then return not_im0 and c:IsHasEffect(EFFECT_NECRO_VALLEY) and c:IsRelateToEffect(re)
    if card::is_controler(d, c, 0) {
        not_im0 && !card::is_has_effect(d, c, EFFECT_NECRO_VALLEY, None).is_empty() && card::is_relate_to_effect(d, c, re)
    // else return not_im1 and c:IsHasEffect(EFFECT_NECRO_VALLEY) and c:IsRelateToEffect(re)
    } else {
        not_im1 && !card::is_has_effect(d, c, EFFECT_NECRO_VALLEY, None).is_empty() && card::is_relate_to_effect(d, c, re)
    }
}
/// `s.discheck(ev,category,re,not_im0,not_im1)`
fn discheck(d: &mut Duel, ev: u32, category: u64, re: EffectId, not_im0: bool, not_im1: bool) -> bool {
    // local ex,tg,ct,p,v=Duel.GetOperationInfo(ev,category)
    // if not ex then return false end
    let Some(info) = duel::get_operation_info(d, ev, category) else { return false };
    let (tg, ct, p, v) = (info.op_cards, info.op_count, info.op_player, info.op_param);
    // if (category==CATEGORY_LEAVE_GRAVE or v==LOCATION_GRAVE) and ct>0 and not tg then
    if (category == CATEGORY_LEAVE_GRAVE || v == LOCATION_GRAVE as i32) && ct > 0 && tg.is_none() {
        // if p==0 then return not_im0
        if p == 0 {
            return not_im0;
        // elseif p==1 then return not_im1
        } else if p == 1 {
            return not_im1;
        // elseif p==PLAYER_ALL then return not_im0 or not_im1
        } else if p as u32 == PLAYER_ALL {
            return not_im0 || not_im1;
        // elseif p==PLAYER_EITHER then return not_im0 and not_im1
        } else if p as u32 == PLAYER_EITHER {
            return not_im0 && not_im1;
        }
    }
    // if tg and #tg>0 then
    if let Some(tg) = tg {
        if !tg.is_empty() {
            // return tg:IsExists(s.discheckfilter,1,nil,not_im0,not_im1,re)
            return group::is_exists(d, &tg, &|d, c| discheckfilter(d, c, not_im0, not_im1, re), 1, NO_EX);
        }
    }
    // return false
    false
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    // if not Duel.IsChainDisablable(ev) or re:GetHandler():IsHasEffect(EFFECT_NECRO_VALLEY_IM) then return end
    if !duel::is_chain_disablable(d, a.ev as u8) {
        return;
    }
    let h = effect::get_handler(d, re).nil();
    if !card::is_has_effect(d, h, EFFECT_NECRO_VALLEY_IM, None).is_empty() {
        return;
    }
    // local res=false
    let mut res = false;
    // local not_im0=not Duel.IsPlayerAffectedByEffect(0,EFFECT_NECRO_VALLEY_IM)
    let not_im0 = duel::is_player_affected_by_effect(d, 0, EFFECT_NECRO_VALLEY_IM).is_none();
    // local not_im1=not Duel.IsPlayerAffectedByEffect(1,EFFECT_NECRO_VALLEY_IM)
    let not_im1 = duel::is_player_affected_by_effect(d, 1, EFFECT_NECRO_VALLEY_IM).is_none();
    let ev = a.ev;
    // if not res and s.discheck(ev,CATEGORY_SPECIAL_SUMMON,re,not_im0,not_im1) then res=true end
    if !res && discheck(d, ev, CATEGORY_SPECIAL_SUMMON, re, not_im0, not_im1) {
        res = true;
    }
    // if not res and s.discheck(ev,CATEGORY_REMOVE,re,not_im0,not_im1) then res=true end
    if !res && discheck(d, ev, CATEGORY_REMOVE, re, not_im0, not_im1) {
        res = true;
    }
    // if not res and s.discheck(ev,CATEGORY_TOHAND,re,not_im0,not_im1) then res=true end
    if !res && discheck(d, ev, CATEGORY_TOHAND, re, not_im0, not_im1) {
        res = true;
    }
    // if not res and s.discheck(ev,CATEGORY_TODECK,re,not_im0,not_im1) then res=true end
    if !res && discheck(d, ev, CATEGORY_TODECK, re, not_im0, not_im1) {
        res = true;
    }
    // if not res and s.discheck(ev,CATEGORY_TOEXTRA,re,not_im0,not_im1) then res=true end
    if !res && discheck(d, ev, CATEGORY_TOEXTRA, re, not_im0, not_im1) {
        res = true;
    }
    // if not res and s.discheck(ev,CATEGORY_EQUIP,re,not_im0,not_im1) then res=true end
    if !res && discheck(d, ev, CATEGORY_EQUIP, re, not_im0, not_im1) {
        res = true;
    }
    // if not res and s.discheck(ev,CATEGORY_SET,re,not_im0,not_im1) then res=true end
    if !res && discheck(d, ev, CATEGORY_SET, re, not_im0, not_im1) {
        res = true;
    }
    // if not res and s.discheck(ev,CATEGORY_LEAVE_GRAVE,re,not_im0,not_im1) then res=true end
    if !res && discheck(d, ev, CATEGORY_LEAVE_GRAVE, re, not_im0, not_im1) {
        res = true;
    }
    // if res then Duel.NegateEffect(ev) end
    if res {
        duel::negate_effect(d, ev as u8);
    }
}
/// `s.cannotsptg(e,c,sp,sumtype,sumpos,target_p,sumeff)`
fn cannotsptg(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    // return c:IsLocation(LOCATION_GRAVE) and sumeff and not sumeff:IsActivated() and not sumeff:IsHasProperty(EFFECT_FLAG_CANNOT_DISABLE)
    if !card::is_location(d, c, LOCATION_GRAVE) {
        return false;
    }
    let Some(sumeff) = a.v(6).as_effect() else { return false };
    if effect::is_activated(d, sumeff) || effect::is_has_property(d, sumeff, EFFECT_FLAG_CANNOT_DISABLE, 0) {
        return false;
    }
    // and not Duel.IsPlayerAffectedByEffect(c:GetControler(),EFFECT_NECRO_VALLEY_IM) and not c:IsHasEffect(EFFECT_NECRO_VALLEY_IM)
    let p = card::get_controler(d, c);
    if duel::is_player_affected_by_effect(d, p, EFFECT_NECRO_VALLEY_IM).is_some() || !card::is_has_effect(d, c, EFFECT_NECRO_VALLEY_IM, None).is_empty() {
        return false;
    }
    // and not sumeff:GetHandler():IsHasEffect(EFFECT_NECRO_VALLEY_IM)
    let h = effect::get_handler(d, sumeff).nil();
    card::is_has_effect(d, h, EFFECT_NECRO_VALLEY_IM, None).is_empty()
}
