//! Favorite Contact
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 75047173;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_NEOS], listed_series: &[SET_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff{handler=c,fusfilter=s.spfilter,matfilter=Card.IsAbleToDeck,extrafil=s.fextra,
    //     extraop=s.extraop,value=0,chkf=FUSPROC_NOTFUSION|FUSPROC_LISTEDMATS,stage2=s.stage2}
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| spfilter(d, c))),
            matfilter: Some(fusion::mat_filter_fn(|d, c, _e, _tp, _chk| card::is_able_to_deck(d, c))),
            extrafil: Some(Arc::new(|d: &mut Duel, e: EffectId, tp: u8, mg: &CardSet| fextra(d, e, tp, mg))),
            extraop: Some(fusion::extra_op(|d, e, tc, tp, sg| extraop(d, e, tc, tp, sg))),
            value: Some(0),
            chkf: Some((FUSPROC_NOTFUSION | FUSPROC_LISTEDMATS) as u64),
            stage2: Some(Arc::new(|d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, sg: Option<&CardSet>, chk: i64| stage2(d, e, tc, tp, sg, chk))),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_FUSION) and c:ListsArchetypeAsMaterial(SET_HERO)
    card::is_type(d, c, TYPE_FUSION) && card::lists_archetype_as_material(d, c, &[SET_HERO as u32])
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(aux.NecroValleyFilter(Fusion.IsMonsterFilter(Card.IsAbleToDeck)),tp,LOCATION_GRAVE,0,nil)+
    //     Duel.GetMatchingGroup(aux.FaceupFilter(Fusion.IsMonsterFilter(Card.IsAbleToDeck)),tp,LOCATION_REMOVED,0,nil)
    let mut g = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(|d, c| card::is_monster(d, c) && card::is_able_to_deck(d, c))),
        tp,
        LOCATION_GRAVE,
        0,
        NO_EX,
    );
    let g2 = duel::get_matching_group(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_monster(d, c) && card::is_able_to_deck(d, c))),
        tp,
        LOCATION_REMOVED,
        0,
        NO_EX,
    );
    group::add_card(&mut g, &g2);
    (Some(g), None)
}
/// `s.extraop(e,tc,tp,sg)`
fn extraop(d: &mut Duel, _e: EffectId, _tc: Option<CardId>, tp: u8, sg: &mut CardSet) -> bool {
    // local rg=sg:Filter(Card.IsFacedown,nil)
    let rg = group::filter(d, sg, &|d, c| card::is_facedown(d, c), NO_EX);
    // if #rg>0 then Duel.ConfirmCards(1-tp,rg) end
    if rg.len() > 0 {
        duel::confirm_cards(d, 1 - tp, rg);
    }
    // local gyrmg=sg:Filter(Card.IsLocation,nil,LOCATION_GRAVE|LOCATION_REMOVED)
    let gyrmg = group::filter(d, sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE | LOCATION_REMOVED), NO_EX);
    // if #gyrmg>0 then Duel.HintSelection(gyrmg,true) end
    if gyrmg.len() > 0 {
        duel::hint_selection(d, gyrmg, true);
    }
    // Duel.SendtoDeck(sg,nil,SEQ_DECKBOTTOM,REASON_EFFECT+REASON_MATERIAL+REASON_FUSION)
    duel::send_to_deck(d, sg.clone(), None, SEQ_DECKBOTTOM as i32, REASON_EFFECT + REASON_MATERIAL + REASON_FUSION, None);
    // local ct=Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_DECK)
    let opg = duel::get_operated_group(d);
    let ct = group::filter_count(d, &opg, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
    // if ct>0 then Duel.SortDeckbottom(tp,tp,ct) end
    if ct > 0 {
        duel::sort_deckbottom(d, tp, tp, ct as u16);
    }
    // sg:Clear()
    *sg = CardSet::new();
    true
}
/// `s.stage2(e,tc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, sg: Option<&CardSet>, chk: i64) {
    // if chk==1 and sg:IsExists(Card.IsCode,1,nil,CARD_NEOS,SUMMON_TYPE_FUSION,tp) then
    if chk == 1 && group::is_exists(d, sg.nil(), &|d, c| card::is_code(d, c, &[CARD_NEOS, SUMMON_TYPE_FUSION as u32, tp as u32]), 1, NO_EX) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_TO_DECK)
        effect::set_code(d, e1, EFFECT_CANNOT_TO_DECK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc.nil(), e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e1:SetCode(EFFECT_CANNOT_TO_HAND)
        effect::set_code(d, e1, EFFECT_CANNOT_TO_HAND);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc.nil(), e2, false);
    }
}
