//! Dogmatikalamity
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 31002402;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_DOGMATIKA}
    listed_series: &[SET_DOGMATIKA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual Summon any "Dogmatika" Ritual Monster
    // local e1=Ritual.CreateProc({handler=c,lvtype=RITPROC_EQUAL,filter=...,extrafil=...,extraop=s.extraop,stage2=s.stage2,forcedselection=s.ritcheck})
    let e1 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_EQUAL,
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_DOGMATIKA as u16]))),
            extrafil: Some(Arc::new(|d: &mut Duel, a: &EffArgs| {
                Some(duel::get_matching_group(d, Some(&|d, x| card::has_level(d, x) && card::is_able_to_grave(d, x, None)), a.tp, LOCATION_EXTRA, 0, NO_EX))
            })),
            extraop: Some(Arc::new(extraop)),
            stage2: Some(Arc::new(stage2)),
            forcedselection: Some(Arc::new(ritcheck)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.extraop(mat,e,tp,eg,ep,ev,re,r,rp,tc)`
fn extraop(d: &mut Duel, mat: &CardSet, _a: &EffArgs, _tc: CardId) {
    // local extra_mat=mat:Filter(Card.IsLocation,nil,LOCATION_EXTRA)
    let extra_mat = group::filter(d, mat, &|d, c| card::is_location(d, c, LOCATION_EXTRA), NO_EX);
    // if #extra_mat>0 then
    if !extra_mat.is_empty() {
        // Duel.SendtoGrave(extra_mat,REASON_EFFECT|REASON_MATERIAL|REASON_RITUAL)
        duel::send_to_grave(d, extra_mat, REASON_EFFECT | REASON_MATERIAL | REASON_RITUAL, None, None);
    } else {
        // Duel.ReleaseRitualMaterial(mat)
        duel::release_ritual_material(d, mat, false);
    }
}
/// `s.stage2(mat,e,tp,eg,ep,ev,re,r,rp,tc)`
fn stage2(d: &mut Duel, _mat: &CardSet, a: &EffArgs, _tc: CardId) {
    let (e, tp) = (a.e, a.tp);
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // For the rest of this turn after this card resolves, you cannot Special Summon monsters from the Extra Deck
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.ritcheck(e,tp,g,sc)`: `return extra_ct==0 or #g==1, extra_ct>1 or (extra_ct==1 and #g>1)`
fn ritcheck(d: &mut Duel, _e: EffectId, _tp: u8, g: &CardSet, _sc: CardId) -> (bool, bool) {
    // local extra_ct=g:FilterCount(Card.IsLocation,nil,LOCATION_EXTRA)
    let extra_ct = group::filter_count(d, g, &|d, c| card::is_location(d, c, LOCATION_EXTRA), NO_EX);
    (extra_ct == 0 || g.len() == 1, extra_ct > 1 || (extra_ct == 1 && g.len() > 1))
}
