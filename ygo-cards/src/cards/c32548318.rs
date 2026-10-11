//! Rahu Dracotail
//! scripted by Naim
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 32548318;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRACOTAIL], ..ScriptInfo::DEFAULT };

/// `params` of the Fusion Summon effect: `{handler=c, fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_DRACOTAIL), extrafil=s.extramat, stage2=s.summonlimit, extratg=s.extratg}`
fn fus_params() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_DRACOTAIL as u16]))),
        extrafil: Some(Arc::new(extramat)),
        stage2: Some(Arc::new(summonlimit)),
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    }
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local params={handler=c,...}
    // Fusion Summon 1 "Dragontail" Fusion Monster from your Extra Deck, using monsters from your hand, Deck, and/or field
    // local e1=Fusion.CreateSummonEff(params)
    let e1 = fusion::create_summon_eff(d, c, fus_params());
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.extramat(e,tp,mg)
fn extramat(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToGrave),tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_grave(d, c, None)), tp, LOCATION_DECK, 0, NO_EX);
    (Some(g), None)
}
// function s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,0,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE as u64, None, 0, a.tp, LOCATION_DECK as i32);
    true
}
// function s.summonlimit(e,tc,tp,mg,chk)
fn summonlimit(d: &mut Duel, e: EffectId, _tc: Option<CardId>, tp: u8, _mg: Option<&CardSet>, chk: i64) {
    // if chk==2 then
    if chk == 2 {
        // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
        if !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
            return;
        }
        // --You cannot Special Summon from the Extra Deck for the rest of this turn after this card resolves, except Fusion Monsters
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_FUSION) end)
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_FUSION))));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
