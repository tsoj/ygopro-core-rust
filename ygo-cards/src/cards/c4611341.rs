//! S-Force Mystify
//! Ｓ－Ｆｏｒｃｅ ミスティファイ
use super::prelude::*;

pub const ID: u32 = 4611341;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_S_FORCE], material_setcode: &[SET_S_FORCE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ monsters, including an "S-Force" monster
    // Link.AddProcedure(c,nil,2,3,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(3), Some(std::sync::Arc::new(lcheck)), None, None);
    // While an opponent's monster is in this card's column, your opponent cannot target this card with card effects, also they cannot Special Summon with activated monster effects
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE);
    // e1a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1a:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1a, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetCondition(s.columncon)
    effect::set_condition(d, e1a, Some(condition(columncon)));
    // e1a:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1a, aux::tgoval());
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=Effect.CreateEffect(c)
    let e1b = effect::create_effect(d, c);
    // e1b:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1b, EFFECT_TYPE_FIELD);
    // e1b:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1b, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1b:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1b, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1b, LOCATION_MZONE);
    // e1b:SetTargetRange(0,1)
    effect::set_target_range(d, e1b, 0, 1);
    // e1b:SetCondition(s.columncon)
    effect::set_condition(d, e1b, Some(condition(columncon)));
    // e1b:SetTarget(function(e,c,sump,sumtype,sumpos,targetp,se) return se and se:IsActivated() and se:IsMonsterEffect() end)
    effect::set_target(d, e1b, Some(lua_fn(|d, a: Args| {
        let se = match a.v(6).clone() {
            Value::Effect(se) => se,
            _ => return false,
        };
        effect::is_activated(d, se) && effect::is_monster_effect(d, se)
    })));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // (Quick Effect): You can target 1 monster on the field; move it to another of its controller's Main Monster Zones
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.mvtg)
    effect::set_target(d, e2, Some(cost_target(mvtg)));
    // e2:SetOperation(s.mvop)
    effect::set_operation(d, e2, Some(operation(mvop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsSetCard,1,nil,SET_S_FORCE,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_set_card_ctx(d, c, &[SET_S_FORCE as u16], Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.columnfilter(c,opp)`
fn columnfilter(d: &mut Duel, c: CardId, opp: u8) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsControler(opp)
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, opp)
}
/// `s.columncon(e)`
fn columncon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetColumnGroup():IsExists(s.columnfilter,1,nil,1-e:GetHandlerPlayer())
    let h = effect::get_handler(d, a.e).nil();
    let cg = card::get_column_group(d, h, 0, 0);
    let opp = 1 - effect::get_handler_player(d, a.e);
    group::is_exists(d, &cg, &|d, c| columnfilter(d, c, opp), 1, NO_EX)
}
/// `s.mvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn mvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local own_loc=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and LOCATION_MZONE or 0
    let own_loc = if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 { LOCATION_MZONE } else { 0 };
    // local opp_loc=Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and LOCATION_MZONE or 0
    let opp_loc = if duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0 { LOCATION_MZONE } else { 0 };
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and (chkc:IsControler(tp) and (own_loc>0) or (opp_loc>0)) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && ((card::is_controler(d, chkc, tp) && own_loc > 0) || opp_loc > 0);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,own_loc,opp_loc,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, own_loc, opp_loc, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // Duel.SelectTarget(tp,nil,tp,own_loc,opp_loc,1,1,nil)
    duel::select_target(d, tp, None, tp, own_loc, opp_loc, 1, 1, false, NO_EX);
    true
}
/// `s.mvop(e,tp,eg,ep,ev,re,r,rp)`
fn mvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsImmuneToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_immune_to_effect(d, tc, a.e) {
        return;
    }
    // local player=tc:GetControler()
    let player = card::get_controler(d, tc);
    // if Duel.GetLocationCount(player,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, player, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // local is_own=player==tp
        let is_own = player == tp;
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOZONE as u64);
        // local zone=Duel.SelectDisableField(tp,1,is_own and LOCATION_MZONE or 0,is_own and 0 or LOCATION_MZONE,0)
        let loc1 = if is_own { LOCATION_MZONE } else { 0 };
        let loc2 = if is_own { 0 } else { LOCATION_MZONE };
        let mut zone = duel::select_disable_field(d, tp, 1, loc1, loc2, Some(0), false);
        // if not is_own then zone=zone>>16 end
        if !is_own {
            zone >>= 16;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOZONE as u64);
        // Duel.MoveSequence(tc,math.log(zone,2))
        if zone == 0 {
            // math.log(nil,2) is an error in Lua (the select has no zone)
            crate::lua_error("bad argument #1 to 'log' (number expected, got nil)");
        }
        let seq = (zone as f64).log2() as u8;
        duel::move_sequence(d, tc, seq, None);
    }
}
