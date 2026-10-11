//! Darklord Dance
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 99941223;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DARKLORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 DARK Fairy Fusion Monster from your Extra Deck, by banishing its materials from your hand, field, and/or GY, and if you do, it gains 1000 ATK
    // local e1=Fusion.CreateSummonEff({
    //     handler=c, fusfilter=s.fusfilter, matfilter=Card.IsAbleToRemove, extrafil=s.fextra,
    //     extraop=Fusion.BanishMaterial, extratg=s.extratg, stage2=s.stage2 })
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| fusfilter(d, c))),
            matfilter: Some(fusion::mat_filter_fn(|d, c, _e, tp, chk| card::is_able_to_remove(d, c, None, Some(tp), Some(chk as u32)))),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extraop: Some(fusion::banish_material()),
            extratg: Some(cost_target(extratg)),
            stage2: Some(std::sync::Arc::new(|d: &mut Duel, e: EffectId, fc: Option<CardId>, tp: u8, sg: Option<&CardSet>, chk: i64| {
                stage2(d, e, fc, tp, sg, chk)
            })),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.fusfilter(c)`
fn fusfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsRace(RACE_FAIRY)
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_race(d, c, RACE_FAIRY)
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(0);
    let sg = a.g(1);
    let fc = a.c(2);
    // return fc:IsSetCard(SET_DARKLORD) or not sg:IsExists(Card.IsControler,1,nil,1-tp)
    card::is_set_card(d, fc, &[SET_DARKLORD as u16]) || !group::is_exists(d, &sg, &|d, c| card::is_controler(d, c, 1 - tp), 1, NO_EX)
}
/// `s.fextrafilter(c,tp)`
fn fextrafilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsAbleToRemove() and (c:IsControler(tp) or (c:IsSetCard(SET_DARKLORD) and c:IsFaceup()))
    card::is_monster(d, c)
        && card::is_able_to_remove(d, c, None, None, None)
        && (card::is_controler(d, c, tp) || (card::is_set_card(d, c, &[SET_DARKLORD as u16]) && card::is_faceup(d, c)))
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then return nil,s.fcheck end
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() {
        return (None, Some(lua_fn(fcheck)));
    }
    // return Duel.GetMatchingGroup(s.fextrafilter,tp,LOCATION_GRAVE,LOCATION_MZONE,nil,tp),s.fcheck
    let g = duel::get_matching_group(d, Some(&|d, c| fextrafilter(d, c, tp)), tp, LOCATION_GRAVE, LOCATION_MZONE, NO_EX);
    (Some(g), Some(lua_fn(fcheck)))
}
/// `s.extratg(e,tp,...,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
/// `s.stage2(e,fc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, fc: Option<CardId>, _tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==0 then
    if chk == 0 {
        // It gains 1000 ATK
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // fc:RegisterEffect(e1)
        card::register_effect(d, fc.nil(), e1, false);
    }
}
