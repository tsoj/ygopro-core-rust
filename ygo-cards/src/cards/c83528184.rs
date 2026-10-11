//! GMX Associate Noma
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 83528184;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GMX],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is Special Summoned by a monster effect: You can target 1 card each from your GY and your opponent's GY;
    // place each card on either the top or bottom of the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re and re:IsMonsterEffect() end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If a monster is Normal or Special Summoned to your opponent's field, while this card is in your Monster Zone
    // (except during the Damage Step): You can Fusion Summon 1 "GMX" Fusion Monster from your Extra Deck, by shuffling
    // its materials from your field, GY, and/or banishment into the Deck
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_FUSION_SUMMON+CATEGORY_SPECIAL_SUMMON+CATEGORY_TODECK)
    effect::set_category(d, e2a, CATEGORY_FUSION_SUMMON + CATEGORY_SPECIAL_SUMMON + CATEGORY_TODECK);
    // e2a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    // e2a:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(Card.IsControler,1,nil,1-tp) end)
    effect::set_condition(d, e2a, Some(condition(e2_cond)));
    // e2a:SetTarget(Fusion.SummonEffTG(fusion_params))
    let tg = fusion::summon_eff_tg(d, fusion_params());
    effect::set_target(d, e2a, Some(tg));
    // e2a:SetOperation(Fusion.SummonEffOP(fusion_params))
    let op = fusion::summon_eff_op(d, fusion_params());
    effect::set_operation(d, e2a, Some(op));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return re and re:IsMonsterEffect() end`
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    match a.re {
        Some(re) => effect::is_monster_effect(d, re),
        None => false,
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(Card.IsControler,1,nil,1-tp) end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_controler(d, c, 1 - a.tp), 1, NO_EX)
}
/// `fusion_params` of this card
fn fusion_params() -> fusion::SummonEff {
    fusion::SummonEff {
        // fusfilter=function(c) return c:IsSetCard(SET_GMX) end
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_GMX as u16]))),
        // matfilter=Fusion.OnFieldMat(Card.IsAbleToDeck)
        matfilter: Some(fusion::on_field_mat_with(fusion::mat_filter_fn(|d, c, _e, _tp, _chk| card::is_able_to_deck(d, c)))),
        // extrafil=s.fextra
        extrafil: Some(Arc::new(fextra)),
        // extraop=Fusion.ShuffleMaterial
        extraop: Some(fusion::shuffle_material()),
        // extratg=s.extratg
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    }
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetTargetGroup(Card.IsAbleToDeck,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil)
    let g = duel::get_target_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(
                d,
                &g,
                Some(a.e),
                tp,
                Some(2),
                Some(2),
                Some(&aux::dpcheck(|d, c| card::get_controler(d, c) as i64)),
                0,
                tp,
                None,
                None,
                None,
                false
            ),
            Err(true)
        );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),1,tp,HINTMSG_TODECK)
    let tg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        tp,
        Some(2),
        Some(2),
        Some(&aux::dpcheck(|d, c| card::get_controler(d, c) as i64)),
        1,
        tp,
        Some(HINTMSG_TODECK as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(tg), 2, tp, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local your_g,opp_g=tg:Split(Card.IsControler,nil,tp)
    let (your_g, opp_g) = group::split(d, &tg, &|d, c| card::is_controler(d, c, tp), NO_EX);
    // if #your_g>0 then s.place_on_top_or_bottom(your_g,tp,tp) end
    if !your_g.is_empty() {
        place_on_top_or_bottom(d, your_g, tp, tp);
    }
    // if #opp_g>0 then s.place_on_top_or_bottom(opp_g,1-tp,tp) end
    if !opp_g.is_empty() {
        place_on_top_or_bottom(d, opp_g, 1 - tp, tp);
    }
}
/// `s.place_on_top_or_bottom(group,owner,trig_player)`
fn place_on_top_or_bottom(d: &mut Duel, group: CardSet, owner: u8, trig_player: u8) {
    // local op=nil
    // if Duel.GetFieldGroupCount(owner,LOCATION_DECK,0)==0 then
    let op: usize = if duel::get_field_group_count(d, owner, LOCATION_DECK, 0) == 0 {
        // op=1
        1
    } else {
        // local offset=(owner~=trig_player) and 2 or 0
        let offset = if owner != trig_player { 2 } else { 0 };
        // op=Duel.SelectEffect(trig_player,{true,aux.Stringid(id,2+offset)},{true,aux.Stringid(id,3+offset)})
        duel::select_effect(
            d,
            trig_player,
            &[(true, aux::stringid(ID, 2 + offset)), (true, aux::stringid(ID, 3 + offset))],
        )
        .nil()
    };
    // local sequence=op==1 and SEQ_DECKTOP or SEQ_DECKBOTTOM
    let sequence = if op == 1 { SEQ_DECKTOP } else { SEQ_DECKBOTTOM };
    // Duel.SendtoDeck(group,nil,sequence,REASON_EFFECT)
    duel::send_to_deck(d, group, None, sequence as i32, REASON_EFFECT, None);
    // if op==1 then Duel.ConfirmDecktop(owner,1) end
    if op == 1 {
        duel::confirm_decktop(d, owner, 1);
    }
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_ONFIELD|LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, (LOCATION_ONFIELD | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(aux.NecroValleyFilter(Fusion.IsMonsterFilter(Card.IsFaceup,Card.IsAbleToDeck)),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
    let g = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(|d, c| {
            card::is_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
        })),
        tp,
        LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        NO_EX,
    );
    (Some(g), None)
}
