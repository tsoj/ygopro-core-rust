//! Blue-Eyes Tyrant Dragon
use super::prelude::*;

pub const ID: u32 = 11443677;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_BLUEEYES_W_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,CARD_BLUEEYES_W_DRAGON,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(CARD_BLUEEYES_W_DRAGON),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), st, Some(tp)))),
        ],
    );
    // Alt. Special Summon procedure
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e1, LOCATION_EXTRA);
    effect::set_condition(d, e1, Some(lua_fn(hspcon)));
    effect::set_target(d, e1, Some(lua_fn(hsptg)));
    effect::set_operation(d, e1, Some(lua_fn(hspop)));
    card::register_effect(d, c, e1, false);
    // Unaffected by Trap effects
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    effect::set_value_fn(d, e2, lua_fn(efilter));
    card::register_effect(d, c, e2, false);
    // Multi-attack
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_ATTACK_ALL);
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
    // Set Trap from GY
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_SET);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e4, EVENT_DAMAGE_STEP_END);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_target(d, e4, Some(cost_target(fgtg)));
    effect::set_operation(d, e4, Some(operation(fgop)));
    card::register_effect(d, c, e4, false);
}
/// `s.hspfilter(c,tp,sc)`
fn hspfilter(d: &mut Duel, c: CardId, tp: u8, sc: CardId) -> bool {
    // c:IsCode(CARD_BLUEEYES_W_DRAGON) and c:GetEquipGroup():IsExists(Card.IsOriginalType,1,nil,TYPE_FUSION)
    //   and Duel.GetLocationCountFromEx(tp,tp,c,sc)>0
    if !card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON]) {
        return false;
    }
    let eg = card::get_equip_group(d, c);
    group::is_exists(d, &eg, &|d, x| card::is_original_type(d, x, TYPE_FUSION), 1, NO_EX)
        && duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), duel::ExtraCard::Card(sc), None).0 > 0
}
/// `s.hspcon(e,c)`
fn hspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.CheckReleaseGroup(tp,s.hspfilter,1,false,1,true,c,tp,nil,nil,nil,tp,c)
    let ra = duel::ReleaseArgs { use_hand: Some(false), max: Some(1), check_field: true, to_check: Some(c), toplayer: Some(tp), zone: None, use_oppo: false, cancelable: false };
    duel::check_release_group(d, tp, Some(&|d, x| hspfilter(d, x, tp, c)), 1, ra, NO_EX, false)
}
/// `s.hsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn hsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    // local g=Duel.SelectReleaseGroup(tp,s.hspfilter,1,1,false,true,true,c,tp,nil,false,nil,tp,c)
    let ra = duel::ReleaseArgs { use_hand: Some(false), max: None, check_field: true, to_check: Some(c), toplayer: Some(tp), zone: None, use_oppo: false, cancelable: true };
    let g = duel::select_release_group(d, tp, Some(&|d, x| hspfilter(d, x, tp, c)), 1, 1, ra, NO_EX, false);
    // if g then
    if let Some(g) = g {
        // g:KeepAlive() (groups are values in Rust)
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.hspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn hspop(d: &mut Duel, a: Args) {
    // local g=e:GetLabelObject()
    // if not g then return end
    let Value::Group(g) = effect::get_label_object(d, a.e(0)) else {
        return;
    };
    // Duel.Release(g,REASON_COST|REASON_MATERIAL)
    duel::release(d, g, REASON_COST | REASON_MATERIAL, None);
    // g:DeleteGroup() (nothing in Rust)
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:IsTrapEffect()
    effect::is_trap_effect(d, a.e(1))
}
/// `s.stfilter(c)`
fn stfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTrap() and c:IsSSetable()
    card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.fgtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn fgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.stfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && stfilter(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and Duel.IsExistingTarget(s.stfilter,tp,LOCATION_GRAVE,0,1,nil)
    //   and c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&stfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_PHASE_END&~(RESET_TOGRAVE|RESET_REMOVE|RESET_LEAVE),0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD_PHASE_END & !(RESET_TOGRAVE | RESET_REMOVE | RESET_LEAVE), 0, 1, 0, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectTarget(tp,s.stfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&stfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    true
}
/// `s.fgop(e,tp,eg,ep,ev,re,r,rp)`
fn fgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) and tc:IsSSetable() then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_s_setable(d, tc, false, None) {
            // Duel.SSet(tp,tc)
            duel::s_set(d, tp, tc, None, true);
        }
    }
}
