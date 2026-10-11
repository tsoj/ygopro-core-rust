//! Gem-Knight Dispersion
//! scripted by pyrQ
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 24220368;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[1264319],
    listed_series: &[SET_GEM_KNIGHT, SET_GEM],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn fcheck(d: &mut Duel, a: Args) -> bool {
    // return sg:FilterCount(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)<=2
    let sg = a.g(1);
    group::filter_count(d, &sg, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX) <= 2
}
fn fextrafilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsRace(RACE_ROCK) and c:IsSetCard(SET_GEM_KNIGHT) and c:IsMonster() and c:IsAbleToGrave()
    !card::is_race(d, c, RACE_ROCK as u64)
        && card::is_set_card(d, c, &[SET_GEM_KNIGHT as u16])
        && card::is_monster(d, c)
        && card::is_able_to_grave(d, c, None)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,1264319) then
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[1264319])), tp, LOCATION_GRAVE, 0, 1, NO_EX) {
        // local eg=Duel.GetMatchingGroup(s.fextrafilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,nil)
        let eg = duel::get_matching_group(d, Some(&fextrafilter), tp, LOCATION_DECK | LOCATION_EXTRA, 0, NO_EX);
        // if #eg>0 then return eg,s.fcheck end
        if !eg.is_empty() {
            return (Some(eg), Some(lua_fn(fcheck)));
        }
    }
    // return nil
    (None, None)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GEM) and c:IsMonster() and (c:IsLocation(LOCATION_DECK) or c:IsFaceup()) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GEM as u16])
        && card::is_monster(d, c)
        && (card::is_location(d, c, LOCATION_DECK) || card::is_faceup(d, c))
        && card::is_able_to_hand(d, c, None)
}
fn extratg_fn(d: &mut Duel, a: Args) {
    // if chk==0 then return true end
    if a.int(8) == 0 {
        return;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,0,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 0, a.p(1), (LOCATION_DECK | LOCATION_EXTRA) as i32);
}
fn gem_filter(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    // aux.FilterBoolFunction(Card.IsSetCard,SET_GEM_KNIGHT)
    card::is_set_card(d, c, &[SET_GEM_KNIGHT as u16])
}
/// `params` of the target part: `{fusfilter=..., extrafil=s.fextra, extratg=s.extratg}`
fn tg_params() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(gem_filter)),
        extrafil: Some(Arc::new(fextra)),
        extratg: Some(lua_fn(extratg_fn)),
        ..Default::default()
    }
}
/// `params` of the operation part: `{fusfilter=..., extrafil=s.fextra}`
fn op_params() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(gem_filter)),
        extrafil: Some(Arc::new(fextra)),
        ..Default::default()
    }
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let p = tg_params();
    // local b1=not Duel.HasFlagEffect(tp,id) and Fusion.SummonEffTG(params)(e,tp,eg,ep,ev,re,r,rp,0)
    let b1 = duel::get_flag_effect(d, a.tp, ID) == 0 && {
        let mut a0 = a.clone();
        a0.chk = Some(0);
        fusion::summon_eff_tg_call(d, &p, &a0)
    };
    // local b2=not Duel.HasFlagEffect(tp,id+1) and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_REMOVED,0,1,nil)
    let b2 = duel::get_flag_effect(d, a.tp, ID + 1) == 0
        && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_REMOVED, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil() as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Fusion.SummonEffTG(params)(e,tp,eg,ep,ev,re,r,rp,1)
        let mut a1 = a.clone();
        a1.chk = Some(1);
        fusion::summon_eff_tg_call(d, &p, &a1);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_REMOVED)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_REMOVED) as i32);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Fusion Summon 1 "Gem-Knight" Fusion Monster from your Extra Deck, using monsters from your hand or field as material
        // local params={fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_GEM_KNIGHT),extrafil=s.fextra}
        // Fusion.SummonEffOP(params)(e,tp,eg,ep,ev,re,r,rp)
        let p = op_params();
        fusion::summon_eff_op_call(d, &p, a);
    } else if op == 2 {
        // Add 1 "Gem-" monster from your Deck or banishment to your hand
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK|LOCATION_REMOVED,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g);
        }
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,3))
        effect::set_description(d, e1, aux::stringid(ID, 3));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CHANGE_DAMAGE)
        effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
        // e1:SetTargetRange(0,1)
        effect::set_target_range(d, e1, 0, 1);
        // e1:SetCondition(function() return Duel.IsMainPhase() end)
        effect::set_condition(d, e1, Some(lua_fn(main_phase_cond)));
        // e1:SetValue(function(e,re,val,r,rp,rc) return (r&REASON_EFFECT>0) and val//2 or val end)
        effect::set_value_fn(d, e1, lua_fn(halve_damage));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
fn main_phase_cond(d: &mut Duel, _a: Args) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
fn halve_damage(_d: &mut Duel, a: Args) -> i64 {
    // (r&REASON_EFFECT>0) and val//2 or val
    let r = a.int(3);
    let val = a.int(2);
    if (r & REASON_EFFECT as i64) > 0 { val.div_euclid(2) } else { val }
}
